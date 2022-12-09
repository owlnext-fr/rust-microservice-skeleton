use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HttpInternalServerErrorException {
    pub code: u32,
    pub message: String,
}

impl Default for HttpInternalServerErrorException {
    fn default() -> Self {
        Self {
            code: 500,
            message: "Internal server error".into(),
        }
    }
}
