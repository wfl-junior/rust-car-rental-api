mod controllers;
mod middleware;
mod repositories;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use serde::Serialize;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
  pub pool: PgPool,
}

#[derive(Serialize)]
struct ErrorResponse {
  message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let database_url = dotenv!("DATABASE_URL");
  let app_state = AppState {
    pool: PgPoolOptions::new()
      .max_connections(5)
      .connect(database_url)
      .await
      .unwrap(),
  };

  let port = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse::<u16>()
    .unwrap();

  return HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .app_data(web::Data::new(app_state.clone()))
      .configure(controllers::brands::router)
      .configure(controllers::cars::router)
      .configure(controllers::auth::router)
      .configure(controllers::rentals::router)
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await;
}
