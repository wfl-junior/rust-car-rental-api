use crate::{AppState, ErrorResponse};
use actix_web::{delete, web, HttpResponse, Responder};
use uuid::Uuid;

#[delete("/cars/{id}")]
async fn delete_car_by_id(
  app_state: web::Data<AppState>,
  car_id: web::Path<Uuid>,
) -> impl Responder {
  let result = sqlx::query("DELETE FROM cars WHERE id = $1;")
    .bind(car_id.into_inner())
    .execute(&app_state.pool)
    .await;

  match result {
    Ok(result) => {
      if result.rows_affected() == 0 {
        let response = ErrorResponse {
          message: "Car not found".to_string(),
        };

        return HttpResponse::NotFound().json(response);
      }

      return HttpResponse::NoContent().into();
    }
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
  }
}
