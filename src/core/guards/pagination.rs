use rocket::{
    http::{Header, Status},
    request::{FromRequest, Outcome},
    Request,
};

const DEFAULT_PAGE: u16 = 0;
const DEFAULT_PER_PAGE: u16 = 25;

#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: u16,
    pub per_page: u16,
}

#[derive(Debug)]
pub enum PaginationError {
    PageParseError,
    PerPageParseError,
    ZeroPerPage,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Pagination {
    type Error = PaginationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut page = DEFAULT_PAGE;
        let mut per_page = DEFAULT_PER_PAGE;

        let query_page = req.query_value::<u16>("page");
        let query_per_page = req.query_value::<u16>("per_page");

        if let Some(parsed_page) = query_page {
            if parsed_page.is_err() {
                return Outcome::Failure((Status::BadRequest, PaginationError::PageParseError));
            }

            page = parsed_page.unwrap();
        }

        if let Some(parsed_per_page) = query_per_page {
            if parsed_per_page.is_err() {
                return Outcome::Failure((Status::BadRequest, PaginationError::PerPageParseError));
            }

            per_page = parsed_per_page.unwrap();
        }

        if per_page == 0 {
            return Outcome::Failure((Status::BadRequest, PaginationError::ZeroPerPage));
        }

        Outcome::Success(Pagination { page, per_page })
    }
}
