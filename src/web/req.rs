use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub name: String,
    pub password: String,
}
