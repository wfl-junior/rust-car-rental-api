mod delete;
mod index;
mod show;
mod store;
mod update;

use super::brands::Brand;
use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Car {
  pub id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub brand_id: Uuid,
  pub model: String,
  pub horse_power: i32,
  pub torque_in_lb: f32,
  pub top_speed_in_km: i32,
  pub acceleration_speed_in_km: f32,
  pub weight_in_kg: i32,
  pub rental_price_daily_in_usd: f64,
}

#[derive(Serialize)]
struct CarWithBrandQuery {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  brand_id: Uuid,
  model: String,
  horse_power: i32,
  torque_in_lb: f32,
  top_speed_in_km: i32,
  acceleration_speed_in_km: f32,
  weight_in_kg: i32,
  rental_price_daily_in_usd: f64,

  brand_created_at: DateTime<Utc>,
  brand_updated_at: DateTime<Utc>,
  brand_name: String,
}

#[derive(Serialize)]
struct CarWithBrand {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  brand_id: Uuid,
  model: String,
  horse_power: i32,
  torque_in_lb: f32,
  top_speed_in_km: i32,
  acceleration_speed_in_km: f32,
  weight_in_kg: i32,
  rental_price_daily_in_usd: f64,

  brand: Brand,
}

impl From<CarWithBrandQuery> for CarWithBrand {
  fn from(query: CarWithBrandQuery) -> Self {
    let brand = Brand {
      id: query.brand_id,
      created_at: query.brand_created_at,
      updated_at: query.brand_updated_at,
      name: query.brand_name,
    };

    return CarWithBrand {
      id: query.id,
      created_at: query.created_at,
      updated_at: query.updated_at,
      brand_id: query.brand_id,
      model: query.model,
      horse_power: query.horse_power,
      torque_in_lb: query.torque_in_lb,
      top_speed_in_km: query.top_speed_in_km,
      acceleration_speed_in_km: query.acceleration_speed_in_km,
      weight_in_kg: query.weight_in_kg,
      rental_price_daily_in_usd: query.rental_price_daily_in_usd,
      brand,
    };
  }
}

#[derive(Deserialize)]
struct CarInput {
  brand_id: Uuid,
  model: String,
  horse_power: i32,
  torque_in_lb: f32,
  top_speed_in_km: i32,
  acceleration_speed_in_km: f32,
  weight_in_kg: i32,
  rental_price_daily_in_usd: f64,
}

pub fn router(config: &mut web::ServiceConfig) {
  config
    .service(index::get_all_cars)
    .service(show::get_car_by_id)
    .service(store::create_car)
    .service(update::update_car_by_id)
    .service(delete::delete_car_by_id);
}
