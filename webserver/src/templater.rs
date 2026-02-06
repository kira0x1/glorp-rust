use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub(crate) name: &'a str,
}