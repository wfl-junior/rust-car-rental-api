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

#[derive(Serialize, Clone)]
struct BrandWithCarQuery {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  name: String,

  car_id: Uuid,
  car_created_at: DateTime<Utc>,
  car_updated_at: DateTime<Utc>,
  car_model: String,
  car_horse_power: i32,
  car_torque_in_lb: f32,
  car_top_speed_in_km: i32,
  car_acceleration_speed_in_km: f32,
  car_weight_in_kg: i32,
  car_rental_price_daily_in_usd: f64,
}

#[derive(Serialize, Clone)]
struct BrandWithOptionCarQuery {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  name: String,

  car_id: Option<Uuid>,
  car_created_at: Option<DateTime<Utc>>,
  car_updated_at: Option<DateTime<Utc>>,
  car_model: Option<String>,
  car_horse_power: Option<i32>,
  car_torque_in_lb: Option<f32>,
  car_top_speed_in_km: Option<i32>,
  car_acceleration_speed_in_km: Option<f32>,
  car_weight_in_kg: Option<i32>,
  car_rental_price_daily_in_usd: Option<f64>,
}

#[derive(Serialize)]
struct BrandWithCars {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  name: String,

  cars: Vec<Car>,
}

impl From<BrandWithCarQuery> for BrandWithCars {
  fn from(query: BrandWithCarQuery) -> Self {
    return BrandWithCars {
      id: query.id,
      created_at: query.created_at,
      updated_at: query.updated_at,
      name: query.name,
      cars: Vec::new(),
    };
  }
}

impl From<&BrandWithOptionCarQuery> for BrandWithCars {
  fn from(query: &BrandWithOptionCarQuery) -> Self {
    return BrandWithCars {
      id: query.id,
      created_at: query.created_at,
      updated_at: query.updated_at,
      name: query.name.to_string(),
      cars: Vec::new(),
    };
  }
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
