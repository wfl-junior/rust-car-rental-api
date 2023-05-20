use super::{Car, CarInput};
use crate::{AppState, ErrorResponse};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct CreateCarResponse {
  car: Car,
}

#[post("/cars")]
async fn create_car(
  app_state: web::Data<AppState>,
  input_json: web::Json<CarInput>,
) -> impl Responder {
  let input = input_json.into_inner();
  let result = sqlx::query_as!(
    Car,
    "INSERT INTO cars 
      (brand_id, model, horse_power, torque_in_lb, top_speed_in_km, acceleration_speed_in_km, weight_in_kg, rental_price_daily_in_usd)
    VALUES 
      ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING *;",
    input.brand_id,
    input.model,
    input.horse_power,
    input.torque_in_lb,
    input.top_speed_in_km,
    input.acceleration_speed_in_km,
    input.weight_in_kg,
    input.rental_price_daily_in_usd
  )
  .fetch_one(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let error_message = error.to_string();
      if error_message.contains("unique constraint") {
        let response = ErrorResponse {
          message: format!("Car {} already exists", input.model),
        };

        return HttpResponse::Conflict().json(response);
      }

      let response = ErrorResponse {
        message: error_message,
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(car) => {
      let response = CreateCarResponse { car };
      return HttpResponse::Created().json(response);
    }
  };
}
