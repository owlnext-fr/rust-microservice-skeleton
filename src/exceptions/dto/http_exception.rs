use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct HttpException {
    pub code: u16,
    pub message: String,
}

impl HttpException {
    pub fn from_code(code: u16) -> Self {
        let status: Status = Status::new(code);

        HttpException {
            code: status.code,
            message: status.reason().unwrap_or("Unknown error").to_string(),
        }
    }

    pub fn from_code_with_message(code: u16, message: String) -> Self {
        let mut ex: Self = Self::from_code(code);
        ex.message = message;

        ex
    }

    pub fn get_status(&self) -> Status {
        Status::new(self.code)
    }
}
