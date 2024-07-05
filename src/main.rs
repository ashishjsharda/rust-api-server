mod handlers;
mod models;
mod routes;
mod state;

use actix_web::{App, HttpServer};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app_state = routes::initialize_state();

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::configure)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
