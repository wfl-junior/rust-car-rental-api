mod store;

use actix_web::web;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Rental {
  pub id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub user_id: Uuid,
  pub car_id: Uuid,
  pub starts_at: DateTime<Utc>,
  pub ends_at: DateTime<Utc>,
  pub canceled_at: Option<DateTime<Utc>>,
}

pub fn router(config: &mut web::ServiceConfig) {
  config.service(store::create_rental);
}
