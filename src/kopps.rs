//! Get some relevant data from kopps.
use super::MyResult;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fmt::{self, Display};

pub struct Kopps {
    client: Client,
}

impl Kopps {
    pub fn new() -> MyResult<Kopps> {
        Ok(Kopps {
            client: Client::builder().build()?,
        })
    }
    pub fn get_info(&self, course_code: &str) -> MyResult<SyllabusResultForCourse> {
        Ok(self
            .client
            .get(&format!(
                "https://api.kth.se/api/kopps/v2/syllabuses/{}",
                course_code
            ))
            .header("accept", "application/json")
            .send()?
            .json()?)
    }
}

/// https://api.kth.se/api/kopps/v2/syllabuses/DA226X
#[derive(Clone, Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct SyllabusResultForCourse {
    course: CourseBasics,
    examiners: Vec<User>,
}

impl Display for SyllabusResultForCourse {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let course = &self.course;
        writeln!(
            out,
            "{}; {} {}",
            course.departmentCode, course.courseCode, course.title
        )?;
        write!(out, "  examiners: ")?;
        for u in &self.examiners {
            write!(out, "{}, ", u)?;
        }
        writeln!(out)?;
        if let Some(contact) = &course.infoContactName {
            writeln!(out, "  contact:  {}", contact)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct CourseBasics {
    title: String,
    courseCode: String,
    departmentCode: String,
    infoContactName: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct User {
    // kthid (ignored),
    givenName: String,
    lastName: String,
    email: String,
    // username	(ignored),
}

impl Display for User {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{} {} <{}>", self.givenName, self.lastName, self.email)
    }
}
