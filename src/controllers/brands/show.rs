use super::{BrandWithCarQuery, BrandWithCars, Car};
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct GetBrandByIdResponse {
  brand: BrandWithCars,
}

#[get("/brands/{id}")]
async fn get_brand_by_id(
  app_state: web::Data<AppState>,
  brand_id: web::Path<Uuid>,
) -> impl Responder {
  let result = sqlx::query_as!(
    BrandWithCarQuery,
    "
      SELECT
        brands.*,
        cars.id as car_id,
        cars.created_at as car_created_at,
        cars.updated_at as car_updated_at,
        cars.model as car_model,
        cars.horse_power as car_horse_power,
        cars.torque_in_lb as car_torque_in_lb,
        cars.top_speed_in_km as car_top_speed_in_km,
        cars.acceleration_speed_in_km as car_acceleration_speed_in_km,
        cars.weight_in_kg as car_weight_in_kg,
        cars.rental_price_daily_in_usd as car_rental_price_daily_in_usd
      FROM brands INNER JOIN cars ON brands.id = cars.brand_id
      WHERE brands.id = $1
      ORDER BY cars.created_at ASC;
    ",
    brand_id.into_inner()
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
    Ok(brands_with_car_query) => {
      let mut brand_with_cars: Option<BrandWithCars> = None;

      for query in brands_with_car_query {
        match brand_with_cars.as_mut() {
          None => {
            brand_with_cars = Some(BrandWithCars::from(query));
          }
          Some(brand) => {
            let car = Car {
              id: query.car_id,
              created_at: query.car_created_at,
              updated_at: query.car_updated_at,
              brand_id: query.id,
              model: query.car_model.to_string(),
              horse_power: query.car_horse_power,
              torque_in_lb: query.car_torque_in_lb,
              top_speed_in_km: query.car_top_speed_in_km,
              acceleration_speed_in_km: query.car_acceleration_speed_in_km,
              weight_in_kg: query.car_weight_in_kg,
              rental_price_daily_in_usd: query.car_rental_price_daily_in_usd,
            };

            brand.cars.push(car);
          }
        }
      }

      match brand_with_cars {
        None => {
          let response = ErrorResponse {
            message: "Brand not found".to_string(),
          };

          return HttpResponse::NotFound().json(response);
        }
        Some(brand) => {
          let response = GetBrandByIdResponse { brand };

          return HttpResponse::Ok().json(response);
        }
      }
    }
  }
}
