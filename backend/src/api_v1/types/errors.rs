use actix_web::{
    body::BoxBody, 
    error, 
    http::{
        header::ContentType, 
        StatusCode
    }, 
    HttpResponse
};
use derive_more::derive::{Display, Error};
use crate::api_v1::types::responses::fail::*;

#[derive(Debug, Display, Error)]
pub enum Errors {
    #[display("Page not found")]
    PageNotFound {
        endpoints: Option<&'static [(&'static str, &'static str)]>
    },

    #[display("Bad request: invalid {what_invalid}")]
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

    #[display("Email rate limit hit. Retry in {how_much}s")]
    EmailRateLimit {
        how_much: u32,
        timestamp: u64
    },

    #[display("IP rate limit hit. Retry in {how_much}s")]
    IPRateLimit {
        how_much: u32,
        timestamp: u64
    },

    #[display("Invalid route: the correct route is {correct_route}")]
    InvalidRoute {
        correct_route: &'static str
    },

    #[display("Invalid code")]
    InvalidCode,

    #[display("Code is outdated or invalid token")]
    InvalidToken,

    #[display("The entry for {what} already exists")]
    AlreadyExists {
        what: &'static str
    },

    #[display("The number of attempts has ended. Email blocked for {how_much}s")]
    TriesOut {
        how_much: u32,
        timestamp: u64
    },

    #[display("Invalid email")]
    InvalidEmail,

    #[display("The body payload limit ({bytes_limit} bytes) reached")]
    PayloadTooLarge {
        bytes_limit: usize
    },

    #[display("Requests rate limit hit")]
    RequestsRateLimit
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
            Self::EmailRateLimit { how_much, timestamp } => BoxBody::new(serde_json::to_string(&EmailRateLimitErrorResponse::new(*how_much, *timestamp)).unwrap()),
            Self::IPRateLimit { how_much, timestamp } => BoxBody::new(serde_json::to_string(&IPRateLimitErrorResponse::new(*how_much, *timestamp)).unwrap()),
            Self::InvalidRoute { correct_route } => BoxBody::new(serde_json::to_string(&InvalidRouteErrorResponse::new(correct_route)).unwrap()),
            Self::InvalidCode => BoxBody::new(serde_json::to_string(&InvalidCodeErrorResponse::new()).unwrap()),
            Self::InvalidToken => BoxBody::new(serde_json::to_string(&InvalidTokenErrorResponse::new()).unwrap()),
            Self::AlreadyExists { what } => BoxBody::new(serde_json::to_string(&AlreadyExistsErrorResponse::new(what)).unwrap()),
            Self::TriesOut { how_much, timestamp } => BoxBody::new(serde_json::to_string(&TriesOutErrorResponse::new(*how_much, *timestamp)).unwrap()),
            Self::InvalidEmail => BoxBody::new(serde_json::to_string(&InvalidEmailErrorResponse::new()).unwrap()),
            Self::PayloadTooLarge { bytes_limit } => BoxBody::new(serde_json::to_string(&PayloadTooLargeErrorResponse::new(*bytes_limit)).unwrap()),
            Self::RequestsRateLimit => BoxBody::new(serde_json::to_string(&RequestsRateLimitErrorResponse::new()).unwrap())
        }
    }
}

impl error::ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::PageNotFound { .. } => StatusCode::NOT_FOUND,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            Self::InternalServer { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailRateLimit { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::IPRateLimit { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::InvalidRoute { .. } => StatusCode::CONFLICT,
            Self::InvalidCode => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::BAD_REQUEST,
            Self::AlreadyExists { .. } => StatusCode::CONFLICT,
            Self::TriesOut { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::InvalidEmail { .. } => StatusCode::BAD_REQUEST,
            Self::PayloadTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::RequestsRateLimit { .. } => StatusCode::TOO_MANY_REQUESTS
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.get_response_body())
    }
}
