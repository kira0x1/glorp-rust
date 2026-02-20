use askama::Template;
use std::fmt::Debug;

#[derive(Debug)]
pub struct IconBox<'a> {
    pub message: &'a str,
    pub icon: &'a str,
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct IndexTmpl<'a> {
    pub globe_message: &'a str,
    pub glorp_status: Vec<IconBox<'a>>,
}
