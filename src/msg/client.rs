use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserReq {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum CreateUserResp {
    Success,
    UnavailableUsername,
    InvalidUsername,
    InvalidPassword,
    UnexpectedError,
}

#[derive(Deserialize)]
pub struct CreateSessionReq {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum CreateSessionResp {
    Success,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum DeleteSessionResp {
    Success,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Deserialize)]
pub struct CreateBatchReq {
    pub urls: Vec<url::Url>,
}

#[derive(Serialize)]
pub enum CreateBatchResp {
    Success { batch_uuid: uuid::Uuid },
    InvalidCredentials,
    NoUrls,
    UnexpectedError,
}
