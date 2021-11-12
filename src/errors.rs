use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use log::error;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct InternalError {
    pub status_code: u16,
    pub message: String,
}

impl InternalError {
    pub fn new(status_code: u16, message: String) -> InternalError {
        InternalError {
            status_code: status_code,
            message: message,
        }
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for InternalError {
    fn from(error: DieselError) -> InternalError {
        match error {
            DieselError::DatabaseError(_, err) => {
                InternalError::new(409, err.message().to_string())
            }
            DieselError::NotFound => InternalError::new(404, "Record not found".to_string()),
            err => InternalError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl ResponseError for InternalError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
