use actix_web::http::StatusCode;
use actix_web::{post, web, Responder};
use diesel::result::DatabaseErrorKind;
use diesel_async::RunQueryDsl;

use crate::models::*;
use crate::msg::client;
use crate::schema;
use crate::state::State;

#[post("/user/create")]
pub async fn create_user(state: web::Data<State>, request: web::Json<client::CreateUserReq>) -> impl Responder {
    if request.name.is_empty() {
        return (web::Json(client::CreateUserResp::InvalidUsername), StatusCode::BAD_REQUEST);
    }
    if request.password.is_empty() {
        return (web::Json(client::CreateUserResp::InvalidPassword), StatusCode::BAD_REQUEST);
    }
    let hashed = match bcrypt::hash(&request.password, bcrypt::DEFAULT_COST) {
        Ok(s) => s,
        Err(e) => {
            error!("POST /user/create: {e}");
            return (web::Json(client::CreateUserResp::UnexpectedError), StatusCode::INTERNAL_SERVER_ERROR);
        },
    };
    let new_user = InsUser {
        name: request.name.clone(),
        passhash: hashed,
    };
    let mut conn = match state.pool.get().await {
        Ok(c) => c,
        Err(e) => {
            error!("POST /user/create: {e}");
            return (web::Json(client::CreateUserResp::UnexpectedError), StatusCode::INTERNAL_SERVER_ERROR);
        },
    };
    let count = diesel::insert_into(schema::users::table)
        .values(new_user)
        .execute(&mut conn)
        .await;

    match count {
        Ok(1) => (web::Json(client::CreateUserResp::Success), StatusCode::CREATED),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            (web::Json(client::CreateUserResp::UnavailableUsername), StatusCode::CONFLICT)
        },
        Err(e) => {
            error!("POST /user/create: {e}");
            (web::Json(client::CreateUserResp::UnexpectedError), StatusCode::INTERNAL_SERVER_ERROR)
        },
        _ => unreachable!("≠1 rows affected while creating single user"),
    }
}
