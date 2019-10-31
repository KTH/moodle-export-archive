#![recursion_limit = "512"]
#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::mdl_assign::dsl as a;
use crate::schema::mdl_course::dsl as c;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env::var;

fn main() {
    dotenv().unwrap();
    let db = PgConnection::establish(&var("DATABASE_URL").unwrap())
        .expect("Establish postgres connection");

    let courses = c::mdl_course
        .select((c::id, c::fullname, c::shortname))
        .limit(100)
        .load::<(i64, String, String)>(&db)
        .unwrap();
    for course in courses {
        let ass = a::mdl_assign
            .select((a::id, a::name, a::intro))
            .filter(a::course.eq(course.0))
            .load::<(i64, String, String)>(&db)
            .unwrap();
        if !ass.is_empty() {
            println!("{:?}\n\t{:?}", course, ass);
        }
    }
}
