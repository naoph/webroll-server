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
