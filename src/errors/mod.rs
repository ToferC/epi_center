pub mod error_handler {
    
    use actix_web::http::StatusCode;
    use actix_web::{HttpResponse, ResponseError};
    use diesel::result::Error as DieselError;
    use serde::Deserialize;
    use async_graphql::*;
    use std::fmt;

    #[derive(Debug, Deserialize)]
    pub struct CustomError {
        pub error_status_code: u16,
        pub error_message: String,
    }

    impl CustomError {
        pub fn new(error_status_code: u16, error_message: String) -> Self {
            CustomError {
                error_status_code,
                error_message,
            }
        }
    }

    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(&format!("{} = {}", self.error_message, self.error_status_code))
        }
    }

    impl From<DieselError> for CustomError {
        fn from(error: DieselError) -> CustomError {
            match error {
                DieselError::DatabaseError(_, err) => {
                    CustomError::new(501, err.message().to_string())
                },
                DieselError::NotFound => {
                    CustomError::new(502, "Record not found".to_string())
                },
                err => CustomError::new(500, format!("Unknown Diesel Error: {}", err)),
            }
        }
    }

    impl From<FieldError> for CustomError {
        fn from(error: FieldError) -> CustomError {
            CustomError {
                error_status_code: 511,
                error_message: format!("GraphQL FieldError: {:?}", error).to_string(),
            }
        }
    }

    // Custom Error Codes
    impl ResponseError for CustomError {
        fn error_response(&self) -> HttpResponse {
            let status_code = match StatusCode::from_u16(self.error_status_code) {
                Ok(status_code) => status_code,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };

            match status_code.as_u16() {
                406 => {
                    return HttpResponse::Found().append_header(("Location", "/not_authorized")).finish()
                },
                i if i > 500 => {
                    return HttpResponse::Found().append_header(("Location","/internal_server_error")).finish()
                },
                _ => return HttpResponse::Found().append_header(("Location","/internal_server_error")).finish()
            };
        }
    }
}