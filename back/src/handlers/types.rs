use serde::Deserialize;
#[derive(Deserialize)]
pub struct NewThread {
    pub name: Option<String>,
    pub message: String,
    pub title: Option<String>,
}
#[derive(Deserialize)]
pub struct NewPost {
    pub name: Option<String>,
    pub message: String,
}
