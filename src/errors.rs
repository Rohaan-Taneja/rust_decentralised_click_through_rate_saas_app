use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PersErrors {
    pub message: String,
    pub status: StatusCode,
}

impl PersErrors {
    pub fn new(mes: impl Into<String>, status: StatusCode) -> PersErrors {
        let message = mes.into();
        return PersErrors { message, status };
    }
}


impl IntoResponse for PersErrors {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}
