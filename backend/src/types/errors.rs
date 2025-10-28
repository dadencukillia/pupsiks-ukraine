use actix_web::{body::{BoxBody, MessageBody}, error, http::{header::ContentType, StatusCode}, web::{self, Json}, HttpResponse, HttpResponseBuilder, Responder};
use derive_more::derive::{Display, Error};

use crate::types::responses::{BadRequestErrorResponse, EmailRateLimitErrorResponse, InternalServerErrorResponse, PageNotFoundErrorResponse, ResourceNotFoundErrorResponse};

#[derive(Debug, Display, Error)]
pub enum Errors {
    #[display("Page not found")]
    PageNotFound {
        endpoints: Option<&'static [(&'static str, &'static str)]>
    },

    #[display("Wrong request: invalid {what_invalid}")]
    BadRequest {
        what_invalid: &'static str
    },

    #[display("Resource not found: {what} not found")]
    ResourceNotFound {
        what: &'static str
    },

    #[display("Something went wrong with {what}")]
    InternalServer {
        what: &'static str
    },

    #[display("Email rate limit hit. Retry in {how_much}s.")]
    EmailRateLimit {
        how_much: u32,
        timestamp: u32
    }
}

impl Errors {
    fn get_response_body(&self) -> BoxBody {
        match self {
            Self::PageNotFound { endpoints } => BoxBody::new(serde_json::to_string(&PageNotFoundErrorResponse::new(
                endpoints.map_or(None, |a| Some(PageNotFoundErrorResponse::endpoints_to_vec(a)))
            )).unwrap()),
            Self::BadRequest { what_invalid } => BoxBody::new(serde_json::to_string(&BadRequestErrorResponse::new(what_invalid)).unwrap()),
            Self::ResourceNotFound { what } => BoxBody::new(serde_json::to_string(&ResourceNotFoundErrorResponse::new(what)).unwrap()),
            Self::InternalServer { what } => BoxBody::new(serde_json::to_string(&InternalServerErrorResponse::new(what)).unwrap()),
            Self::EmailRateLimit { how_much, timestamp } => BoxBody::new(serde_json::to_string(&EmailRateLimitErrorResponse::new(*how_much, *timestamp)).unwrap())
        }
    }
}

impl error::ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::PageNotFound { endpoints: _ } => StatusCode::NOT_FOUND,
            Self::BadRequest { what_invalid: _ } => StatusCode::BAD_REQUEST,
            Self::ResourceNotFound { what: _ } => StatusCode::NOT_FOUND,
            Self::InternalServer { what: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailRateLimit { how_much: _, timestamp: _ } => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.get_response_body())
    }
}
