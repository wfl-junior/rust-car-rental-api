mod cancel;
mod index;
mod mine;
mod store;

use super::{brands::Brand, cars::CarWithBrand};
use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize)]
struct Rental {
  id: Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  user_id: Uuid,
  car_id: Uuid,
  starts_at: DateTime<Utc>,
  ends_at: DateTime<Utc>,
  canceled_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct RentalsQueryParams {
  car_id: Option<Uuid>,
  starts_at: Option<DateTime<Utc>>,
  ends_at: Option<DateTime<Utc>>,
}

#[derive(FromRow)]
pub struct RentalWithCarAndBrandQuery {
  pub id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub car_id: Uuid,
  pub starts_at: DateTime<Utc>,
  pub ends_at: DateTime<Utc>,
  pub canceled_at: Option<DateTime<Utc>>,

  pub car_created_at: DateTime<Utc>,
  pub car_updated_at: DateTime<Utc>,
  pub car_brand_id: Uuid,
  pub car_model: String,
  pub car_horse_power: i32,
  pub car_torque_in_lb: f32,
  pub car_top_speed_in_km: i32,
  pub car_acceleration_speed_in_km: f32,
  pub car_weight_in_kg: i32,
  pub car_rental_price_daily_in_usd: f64,
  pub car_brand_created_at: DateTime<Utc>,
  pub car_brand_updated_at: DateTime<Utc>,
  pub car_brand_name: String,
}

#[derive(Serialize)]
pub struct RentalWithCarAndBrand {
  pub id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub car_id: Uuid,
  pub starts_at: DateTime<Utc>,
  pub ends_at: DateTime<Utc>,
  pub canceled_at: Option<DateTime<Utc>>,

  pub car: CarWithBrand,
}

impl From<RentalWithCarAndBrandQuery> for RentalWithCarAndBrand {
  fn from(query: RentalWithCarAndBrandQuery) -> Self {
    let brand = Brand {
      id: query.car_brand_id,
      created_at: query.car_brand_created_at,
      updated_at: query.car_brand_updated_at,
      name: query.car_brand_name,
    };

    let car = CarWithBrand {
      id: query.car_id,
      created_at: query.car_created_at,
      updated_at: query.car_updated_at,
      brand_id: query.car_brand_id,
      model: query.car_model,
      horse_power: query.car_horse_power,
      torque_in_lb: query.car_torque_in_lb,
      top_speed_in_km: query.car_top_speed_in_km,
      acceleration_speed_in_km: query.car_acceleration_speed_in_km,
      weight_in_kg: query.car_weight_in_kg,
      rental_price_daily_in_usd: query.car_rental_price_daily_in_usd,
      brand,
    };

    return RentalWithCarAndBrand {
      id: query.id,
      created_at: query.created_at,
      updated_at: query.updated_at,
      car_id: query.car_id,
      starts_at: query.starts_at,
      ends_at: query.ends_at,
      canceled_at: query.canceled_at,
      car,
    };
  }
}

pub fn router(config: &mut web::ServiceConfig) {
  config
    .service(index::get_all_rentals)
    .service(mine::get_all_of_my_rentals)
    .service(store::create_rental)
    .service(cancel::cancel_rental);
}
