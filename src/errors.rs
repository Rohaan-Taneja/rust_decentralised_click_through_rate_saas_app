use axum::http::StatusCode;

#[derive(Debug, Clone)]
pub struct PersErrors {
    pub message: String,
    pub status: StatusCode
    
}

impl PersErrors {
    pub fn new(mes: impl Into<String>, status: StatusCode) -> PersErrors {
        let message = mes.into();
        return PersErrors { message, status };
    }
}
