use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum CreateUserResp {
    Success,
    UnavailableUsername,
    InvalidUsername,
    InvalidPassword,
    UnexpectedError,
}

#[derive(Debug, Serialize)]
pub struct CreateCaptureResp {
    pub uuid: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum MonitorCaptureResp {
    NoSuchCapture,
    Capturing { progress: crate::state::JobProgress },
}
