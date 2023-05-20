use super::{Car, CarInput};
use crate::{AppState, ErrorResponse};
use actix_web::{put, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct UpdateCarByIdResponse {
  car: Car,
}

#[put("/cars/{id}")]
async fn update_car_by_id(
  app_state: web::Data<AppState>,
  car_id: web::Path<Uuid>,
  input_json: web::Json<CarInput>,
) -> impl Responder {
  let input = input_json.into_inner();
  let result = sqlx::query_as!(
    Car,
    "UPDATE cars SET
      brand_id = $2,
      model = $3,
      horse_power = $4,
      torque_in_lb = $5,
      top_speed_in_km = $6,
      acceleration_speed_in_km = $7,
      weight_in_kg = $8,
      rental_price_daily_in_usd = $9,
      updated_at = $10
    WHERE id = $1 RETURNING *",
    car_id.into_inner(),
    input.brand_id,
    input.model,
    input.horse_power,
    input.torque_in_lb,
    input.top_speed_in_km,
    input.acceleration_speed_in_km,
    input.weight_in_kg,
    input.rental_price_daily_in_usd,
    Utc::now(),
  )
  .fetch_optional(&app_state.pool)
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
    Ok(None) => {
      let response = ErrorResponse {
        message: "Car not found".to_string(),
      };

      return HttpResponse::NotFound().json(response);
    }
    Ok(Some(car)) => {
      let response = UpdateCarByIdResponse { car };

      return HttpResponse::Ok().json(response);
    }
  }
}
