mod login;
mod me;
mod register;

use actix_web::web;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct User {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  name: String,
  email: String,
}

#[derive(Serialize)]
struct UserWithPassword {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  name: String,
  email: String,
  password: String,
}

pub fn router(config: &mut web::ServiceConfig) {
  config
    .service(register::register)
    .service(login::login)
    .service(me::get_me);
}
