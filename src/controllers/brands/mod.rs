mod delete;
mod index;
mod show;
mod store;
mod update;

use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
struct Brand {
  id: Uuid,
  name: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct BrandInput {
  name: String,
}

async fn get_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Brand>> {
  return sqlx::query_as!(Brand, "SELECT * FROM brands WHERE id = $1;", id)
    .fetch_optional(pool)
    .await;
}

pub fn router(config: &mut web::ServiceConfig) {
  config
    .service(index::get_all_brands)
    .service(show::get_brand_by_id)
    .service(store::create_brand)
    .service(update::update_brand_by_id)
    .service(delete::delete_brand_by_id);
}
