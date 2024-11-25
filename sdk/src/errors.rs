
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("requestId:{request_id:?}, http_status:{http_status:?}, code:{code:?}, message:{message:?}")]
    APIError {
        request_id: String,
        http_status: u16,
        code: String,
        message: String,
    },
    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),
    #[error(transparent)]
    OtherErrors(#[from] anyhow::Error),
}
