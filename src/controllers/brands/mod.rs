mod delete;
mod index;
mod show;
mod store;
mod update;

use super::cars::Car;
use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Brand {
  pub id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub name: String,
}

#[derive(Deserialize)]
struct BrandInput {
  name: String,
}

pub fn router(config: &mut web::ServiceConfig) {
  config
    .service(index::get_all_brands)
    .service(show::get_brand_by_id)
    .service(store::create_brand)
    .service(update::update_brand_by_id)
    .service(delete::delete_brand_by_id);
}
