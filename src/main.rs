#![recursion_limit = "256"]
#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::mdl_assign::dsl as a;
use crate::schema::mdl_course::dsl as c;
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use slug::slugify;
use std::env::var;
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    dotenv().unwrap();

    let basedir = PathBuf::from("/tmp/moodleexport");
    create_dir_all(&basedir).unwrap();

    let db = PgConnection::establish(&var("DATABASE_URL").unwrap())
        .expect("Establish postgres connection");

    let courses = c::mdl_course
        .select((c::id, c::fullname, c::shortname))
        .load::<(i64, String, String)>(&db)
        .unwrap();
    for course in courses {
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
                        time: Utc
                            .timestamp(modified, 0)
                            .to_rfc3339_opts(SecondsFormat::Secs, true),
                    };
                    let mut data_file =
                        File::create(path.join(&meta.name)).expect("Create archive file");
                    write!(
                        data_file,
                        "{}\n\n{}\n\ndue date: {}.\n",
                        meta.desc,
                        content,
                        Utc.timestamp(due, 0)
                            .to_rfc3339_opts(SecondsFormat::Secs, true),
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
    println!("Course code missing in {:?}", s);
    None
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
