#![recursion_limit = "256"]
#[macro_use]
extern crate diesel;

mod kopps;
mod schema;

use crate::schema::mdl_assign::dsl as a;
use crate::schema::mdl_course::dsl as c;
use chrono::prelude::*;
use diesel::dsl::count_star;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use kopps::Kopps;
use slug::slugify;
use std::env::{args, var};
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    dotenv().unwrap();
    let db = PgConnection::establish(&var("DATABASE_URL").unwrap())
        .expect("Establish postgres connection");

    let mut cmd = args();
    let cmd0 = cmd.next().unwrap();
    match (cmd.next().as_deref(), cmd.next().as_deref()) {
        (Some("dump"), None) => makedump(db),
        (Some("exjobb"), None) => find_exjobbs(db),
        x => eprintln!("Run {} dump or exjobb, got {:?}", cmd0, x),
    }
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
type DbResult<T> = Result<T, diesel::result::Error>;

fn find_exjobbs(db: PgConnection) {
    let kopps = Kopps::new().unwrap();

    for course in list_courses(&db).unwrap() {
        let course_code = get_course_code(&course.2).unwrap_or("unknown");
        if !course_code.ends_with("X") && !course_code.ends_with("x") {
            continue;
        }
        let ass = a::mdl_assign
            .select(count_star())
            .filter(a::course.eq(course.0))
            .first::<i64>(&db)
            .unwrap();
        use crate::schema::mdl_turnitintooltwo_courses::dsl as tt;
        let turnit = tt::mdl_turnitintooltwo_courses
            .select(count_star())
            .filter(tt::courseid.eq(course.0))
            .first::<i64>(&db)
            .unwrap();
        if ass == 0 && turnit == 0 {
            continue;
        }

        println!(
            "{}  ({} assignemnts, {} turnitin course)",
            kopps.get_info(course_code).unwrap(),
            ass,
            turnit,
        );
        println!();
    }
}

fn list_courses(db: &PgConnection) -> DbResult<Vec<(i64, String, String)>> {
    c::mdl_course
        .select((c::id, c::fullname, c::shortname))
        .load::<(i64, String, String)>(db)
}

fn makedump(db: PgConnection) {
    let basedir = PathBuf::from("/tmp/moodleexport");
    create_dir_all(&basedir).unwrap();

    for course in list_courses(&db).unwrap() {
        let ass = a::mdl_assign
            .select((a::id, a::name, a::intro, a::duedate, a::timemodified))
            .filter(a::course.eq(course.0))
            .load::<(i64, String, String, i64, i64)>(&db)
            .unwrap();
        if !ass.is_empty() {
            let course_code = get_course_code(&course.2).unwrap_or("unknown");
            let path = basedir.join(&format!("{}-{}", course_code, course.0));
            create_dir_all(&path).unwrap();
            println!("{} {:?}", course_code, course);
            let mut files = Vec::new();
            for (id, desc, content, due, modified) in ass.into_iter() {
                if !content.is_empty() {
                    let meta = FileMeta {
                        name: format!("{:04}-{}.txt", id, slugify(&desc)),
                        desc,
                        time: Utc.timestamp(modified, 0).fmt_lts(),
                    };
                    let mut data_file =
                        File::create(path.join(&meta.name)).expect("Create archive file");
                    write!(
                        data_file,
                        "{}\n\n{}\n\ndue date: {}.\n",
                        meta.desc,
                        content,
                        Utc.timestamp(due, 0).fmt_lts(),
                    )
                    .expect("Write archive file");
                    files.push(meta);
                } else {
                    println!("\t{}: {:?}", id, desc);
                }
            }
            println!("\t{} files to archive", files.len());
            let mut meta = Vec::new();
            templates::data_xml(&mut meta, course_code, &course.1, &course.2, &files).unwrap();
            fs::write(path.join("meta.xml"), meta).unwrap();
        }
    }
}

pub struct FileMeta {
    name: String,
    desc: String,
    time: String,
}

fn get_course_code(s: &str) -> Option<&str> {
    let mut parts = s.split('/');
    if parts.next() == Some("") && parts.next() == Some("social") && parts.next() == Some("course")
    {
        return parts.next();
    }
    //println!("Course code missing in {:?}", s);
    None
}

/// Format objects the way the KTH Long Time Storag wants them.
///
/// For a time, this in UTC like "2019-11-07T12:00.00", similar to RFC
/// 3339, buth without trailing "Z" or timezone offset.
trait FmtLts {
    /// Format this thing the way the KTH Long Time Storage wants it.
    fn fmt_lts(&self) -> String;
}

impl FmtLts for DateTime<Utc> {
    fn fmt_lts(&self) -> String {
        let mut text = self.to_rfc3339_opts(SecondsFormat::Secs, true);
        if text.ends_with("Z") {
            text.pop();
            text
        } else {
            panic!("Unexpected date format: {:?}", text);
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
