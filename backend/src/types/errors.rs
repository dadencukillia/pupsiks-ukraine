use actix_web::{body::{BoxBody, MessageBody}, error, http::{header::ContentType, StatusCode}, web::{self, Json}, HttpResponse, HttpResponseBuilder, Responder};
use derive_more::derive::{Display, Error};

use crate::types::responses::NotFoundErrorResponse;

#[derive(Debug, Display, Error)]
pub enum Errors {
    #[display("Page not found")]
    NotFound {
        endpoints: Option<&'static [(&'static str, &'static str)]>
    }
}

impl Errors {
    fn get_response_body(&self) -> BoxBody {
        match self {
            Self::NotFound { endpoints } => BoxBody::new(serde_json::to_string(&NotFoundErrorResponse::from(*endpoints)).unwrap())
        }
    }
}

impl error::ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound { endpoints: _ } => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.get_response_body())
    }
}
