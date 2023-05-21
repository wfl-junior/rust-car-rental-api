use super::{RentalWithCarAndBrand, RentalWithCarAndBrandQuery};
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct GetAllRentalsResponse {
  rentals: Vec<RentalWithCarAndBrand>,
}

#[get("/rentals")]
async fn get_all_rentals(app_state: web::Data<AppState>) -> impl Responder {
  let result = sqlx::query_as!(
    RentalWithCarAndBrandQuery,
    "
      SELECT
        rentals.id AS id,
        rentals.created_at AS created_at,
        rentals.updated_at AS updated_at,
        rentals.car_id AS car_id,
        rentals.starts_at AS starts_at,
        rentals.ends_at AS ends_at,
        rentals.canceled_at AS canceled_at,
        cars.created_at AS car_created_at,
        cars.updated_at AS car_updated_at,
        cars.brand_id AS car_brand_id,
        cars.model AS car_model,
        cars.horse_power AS car_horse_power,
        cars.torque_in_lb AS car_torque_in_lb,
        cars.top_speed_in_km AS car_top_speed_in_km,
        cars.acceleration_speed_in_km AS car_acceleration_speed_in_km,
        cars.weight_in_kg AS car_weight_in_kg,
        cars.rental_price_daily_in_usd AS car_rental_price_daily_in_usd,
        brands.created_at AS car_brand_created_at,
        brands.updated_at AS car_brand_updated_at,
        brands.name AS car_brand_name
      FROM rentals
        INNER JOIN cars ON cars.id = rentals.car_id
        INNER JOIN brands ON brands.id = cars.brand_id
      ORDER BY rentals.created_at ASC;
    "
  )
  .fetch_all(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(rentals_with_car_query) => {
      let rentals_with_car: Vec<RentalWithCarAndBrand> = rentals_with_car_query
        .into_iter()
        .map(RentalWithCarAndBrand::from)
        .collect();

      let response = GetAllRentalsResponse {
        rentals: rentals_with_car,
      };

      return HttpResponse::Ok().json(response);
    }
  }
}
