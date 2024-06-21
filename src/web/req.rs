use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCaptureReq {
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct MonitorCaptureReq {
    pub uuid: String,
}
