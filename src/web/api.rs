use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::{delete, post, web, Either, HttpRequest, Responder};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel_async::RunQueryDsl;

use crate::models::*;
use crate::msg::client;
use crate::schema::{self, *};
use crate::state::State;

/// Authenticate the request using the user and session cookies
macro_rules! auth {
    ( $r:expr, $s:expr, $f:expr ) => {{
        let forbidden_response = web::Json($f)
            .customize()
            .with_status(StatusCode::FORBIDDEN);
        let user_id = match $r.cookie("user") {
            Some(a) => a.value().to_string(),
            None => return Either::Right(forbidden_response),
        };
        let user_id = match user_id.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Either::Right(forbidden_response),
        };
        let token = match $r.cookie("session") {
            Some(a) => a.value().to_string(),
            None => return Either::Right(forbidden_response),
        };
        if !$s.validate(user_id, &token).await {
            return Either::Right(forbidden_response);
        };
        user_id
    }};
}

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

#[post("/user/login")]
pub async fn create_session(state: web::Data<State>, request: web::Json<client::CreateSessionReq>) -> impl Responder {
    // Connect to database
    let mut conn = match state.pool.get().await {
        Ok(c) => c,
        Err(e) => {
            error!("POST /user/login: {e}");
            return web::Json(client::CreateSessionResp::UnexpectedError).customize().with_status(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    // Lookup user
    let user: Result<DbUser, _> = users::table
        .filter(users::name.eq(&request.name))
        .first(&mut conn)
        .await;
    let user = match user {
        Ok(u) => u,
        Err(diesel::result::Error::NotFound) => return web::Json(client::CreateSessionResp::InvalidCredentials).customize().with_status(StatusCode::FORBIDDEN),
        Err(e) => {
            error!("POST /user/login: {e}");
            return web::Json(client::CreateSessionResp::UnexpectedError).customize().with_status(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    // Verify password
    match bcrypt::verify(&request.password, &user.passhash) {
        Ok(true) => {},
        Ok(false) => return web::Json(client::CreateSessionResp::InvalidCredentials).customize().with_status(StatusCode::FORBIDDEN),
        Err(e) => {
            error!("POST /user/login: {e}");
            return web::Json(client::CreateSessionResp::UnexpectedError).customize().with_status(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Authentication succeeded, create token
    let token = state.sessions.create(&user).await;

    web::Json(client::CreateSessionResp::Success)
        .customize()
        .add_cookie(&Cookie::new("user", user.id.to_string()))
        .add_cookie(&Cookie::new("session", token))
        .with_status(StatusCode::CREATED)
}

#[delete("/user/session/all")]
pub async fn delete_all_sessions(state: web::Data<State>, full_request: HttpRequest) -> impl Responder {
    // Validate and authenticate session info
    let user_id = auth!(full_request, state.sessions, client::DeleteSessionResp::InvalidCredentials);

    state.sessions.delete_all(user_id).await;

    let response = web::Json(client::DeleteSessionResp::Success)
        .customize()
        .with_status(StatusCode::OK);
    Either::Left(response)
}
