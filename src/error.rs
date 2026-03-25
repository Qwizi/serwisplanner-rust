use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum SWError {
    #[error("HTTP {status_code}: {message}")]
    Http {
        status_code: u16,
        message: String,
        response_data: Value,
    },

    #[error("Authentication error (401): {message}")]
    Authentication {
        message: String,
        response_data: Value,
    },

    #[error("Forbidden (403): {message}")]
    Forbidden {
        message: String,
        response_data: Value,
    },

    #[error("Not found (404): {message}")]
    NotFound {
        message: String,
        response_data: Value,
    },

    #[error("Validation error (422): {message}")]
    Validation {
        message: String,
        response_data: Value,
        errors: Value,
    },

    #[error("Rate limit (429): {message}")]
    RateLimit {
        message: String,
        response_data: Value,
    },

    #[error("Server error ({status_code}): {message}")]
    Server {
        status_code: u16,
        message: String,
        response_data: Value,
    },

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SWError>;

fn extract_message(data: &Value) -> String {
    data.get("message")
        .or_else(|| data.get("error"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown error")
        .to_string()
}

pub(crate) fn raise_for_status(status: u16, data: Value) -> SWError {
    let message = extract_message(&data);
    match status {
        401 => SWError::Authentication {
            message,
            response_data: data,
        },
        403 => SWError::Forbidden {
            message,
            response_data: data,
        },
        404 => SWError::NotFound {
            message,
            response_data: data,
        },
        422 => {
            let errors = data
                .get("errors")
                .cloned()
                .unwrap_or_else(|| data.clone());
            SWError::Validation {
                message,
                response_data: data,
                errors,
            }
        }
        429 => SWError::RateLimit {
            message,
            response_data: data,
        },
        500..=599 => SWError::Server {
            status_code: status,
            message,
            response_data: data,
        },
        _ => SWError::Http {
            status_code: status,
            message,
            response_data: data,
        },
    }
}
