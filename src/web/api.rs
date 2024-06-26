use actix_web::{get, post, web, Responder};
use diesel::result::DatabaseErrorKind;
use diesel_async::RunQueryDsl;

use crate::models::*;
use crate::schema;
use crate::state::State;

use super::req::*;
use super::resp::*;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    "pong"
}

#[post("/user/create")]
pub async fn user_create(data: web::Data<State>, request: web::Json<CreateUserReq>) -> impl Responder {
    if request.name.len() == 0 {
        return web::Json(CreateUserResp::InvalidUsername);
    }
    if request.password.len() == 0 {
        return web::Json(CreateUserResp::InvalidPassword);
    }
    let hashed = match bcrypt::hash(&request.password, bcrypt::DEFAULT_COST) {
        Ok(s) => s,
        Err(e) => {
            error!("POST /user/create: {e}");
            return web::Json(CreateUserResp::UnexpectedError);
        },
    };
    let new_user = InsUser {
        name: request.name.clone(),
        passhash: hashed,
    };
    let mut conn = match data.pool.get().await {
        Ok(c) => c,
        Err(e) => {
            error!("POST /user/create: {e}");
            return web::Json(CreateUserResp::UnexpectedError);
        },
    };
    let count = diesel::insert_into(schema::users::table)
        .values(new_user)
        .execute(&mut conn)
        .await;
    match count {
        Ok(1) => web::Json(CreateUserResp::Success),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            web::Json(CreateUserResp::UnavailableUsername)
        },
        Err(e) => {
            error!("POST /user/create: {e}");
            web::Json(CreateUserResp::UnexpectedError)
        },
        _ => unreachable!("≠1 rows affected while creating single user"),
    }
}

#[post("/capture/create")]
pub async fn capture_create(data: web::Data<State>, request: web::Json<CreateCaptureReq>) -> impl Responder {
    let uuid = uuid::Uuid::new_v4().to_string();
    let url = request.url.clone();
    data.job_manager.new_job(uuid.clone()).await;
    let worker = data.worker_selector.select_worker();
    tokio::task::spawn(crate::extract::extract(url, uuid.clone(), worker.clone()));
    web::Json(CreateCaptureResp { uuid: uuid.to_string() })
}

#[post("/capture/monitor")]
pub async fn capture_monitor(data: web::Data<State>, request: web::Json<MonitorCaptureReq>) -> impl Responder {
    let uuid = request.uuid.clone();
    let response = match data.job_manager.get_progress(&uuid).await {
        Some(p) => MonitorCaptureResp::Capturing { progress: p },
        None => MonitorCaptureResp::NoSuchCapture,
    };
    web::Json(response)
}
