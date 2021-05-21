use serde::{
    Serialize,
    Deserialize
};
use actix_web::HttpResponse;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
    DatabaseError,
    ParseError,
    ValueError,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub error_type: ErrorType, 
}

impl From<diesel::result::Error> for ApiError {
    
    fn from(err: diesel::result::Error) -> ApiError {
        ApiError {
            code: 500,
            message: format!("{}", err),
            error_type: ErrorType::DatabaseError,
        }
    }
}

impl From<r2d2::Error> for ApiError {

    fn from(err:  r2d2::Error) -> ApiError {
        ApiError {
            code: 500,
            message: format!("{}", err),
            error_type: ErrorType::DatabaseError,
        }
    }
}

impl From<chrono::ParseError> for ApiError {

    fn from(err: chrono::ParseError) -> ApiError {
        ApiError {
            code: 500,
            message: format!("{}", err),
            error_type: ErrorType::ParseError,
        }
    }
}

impl std::fmt::Display for ApiError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl actix_web::ResponseError for ApiError {

    fn status_code(&self) -> actix_web::http::StatusCode {
         actix_web::http::StatusCode::from_u16(self.code).unwrap()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(json!(self))
    }
}