use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HttpNotFoundException {
    pub code: u32,
    pub message: String,
}

impl Default for HttpNotFoundException {
    fn default() -> Self {
        Self {
            code: 404,
            message: "Not found".into(),
        }
    }
}
