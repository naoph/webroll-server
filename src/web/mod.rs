mod api;

use actix_web::{HttpServer, App, web};

pub async fn run(host: impl ToString, port: u16, state: crate::state::State) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(api::create_user)
            .service(api::create_session)
            .service(api::delete_all_sessions)
    })
    .bind((host.to_string(), port))?
    .run()
    .await
}
