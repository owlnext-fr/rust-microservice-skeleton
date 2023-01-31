use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct HttpException {
    pub code: u16,
    pub message: String,
    pub errors: Option<String>,
}

impl HttpException {
    pub fn from_code(code: u16) -> Self {
        let status: Status = Status::new(code);

        HttpException {
            code: status.code,
            message: status.reason().unwrap_or("Unknown error").to_string(),
            errors: None,
        }
    }

    pub fn from_status(status: Status) -> Self {
        HttpException {
            code: status.code,
            message: status.reason().unwrap_or("Unknown error").to_string(),
            errors: None,
        }
    }

    pub fn from_code_with_reason(code: u16, reason: Option<String>) -> Self {
        let mut ex: Self = Self::from_code(code);
        ex.errors = reason;

        ex
    }

    pub fn from_status_with_reason(status: Status, reason: Option<String>) -> Self {
        let mut ex = Self::from_status(status);
        ex.errors = reason;

        ex
    }

    pub fn get_status(&self) -> Status {
        Status::new(self.code)
    }
}
