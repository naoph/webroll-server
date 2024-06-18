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
