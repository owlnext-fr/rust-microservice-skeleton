use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::core::response::ErrorMessage;

/// Default page number for pagination.
const DEFAULT_PAGE: u16 = 0;
/// Default number of item per page for pagination.
const DEFAULT_PER_PAGE: u16 = 25;

/// A struct representing pagination options for listings.
#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: u16,
    pub per_page: u16,
}

/// Different states of errors that can encounter pagination extraction.
#[derive(Debug)]
pub enum PaginationError {
    /// the page parameter is invalid.
    PageParseError,
    /// the per_page parameter is invalid.
    PerPageParseError,
    /// the page is <= 0
    ZeroPage,
    /// the per_page is <= 0
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
                req.local_cache(|| ErrorMessage {
                    message: "page parameter cannot be parsed".into(),
                });
                return Outcome::Failure((Status::BadRequest, PaginationError::PageParseError));
            }

            page = parsed_page.unwrap();
        }

        if let Some(parsed_per_page) = query_per_page {
            if parsed_per_page.is_err() {
                req.local_cache(|| ErrorMessage {
                    message: "per_page parameter cannot be parsed".into(),
                });
                return Outcome::Failure((Status::BadRequest, PaginationError::PerPageParseError));
            }

            per_page = parsed_per_page.unwrap();
        }

        if page == 0 {
            req.local_cache(|| ErrorMessage {
                message: "page must be a positive integer".into(),
            });
            return Outcome::Failure((Status::BadRequest, PaginationError::ZeroPerPage));
        }

        if per_page == 0 {
            req.local_cache(|| ErrorMessage {
                message: "per_page must be a positive integer".into(),
            });
            return Outcome::Failure((Status::BadRequest, PaginationError::ZeroPerPage));
        }

        Outcome::Success(Pagination { page, per_page })
    }
}
