use super::{Brand, BrandInput};
use crate::{AppState, ErrorResponse};
use actix_web::{put, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct UpdateBrandByIdResponse {
  brand: Brand,
}

#[put("/brands/{id}")]
async fn update_brand_by_id(
  app_state: web::Data<AppState>,
  brand_id: web::Path<Uuid>,
  input_json: web::Json<BrandInput>,
) -> impl Responder {
  let input = input_json.into_inner();
  let result = sqlx::query_as!(
    Brand,
    "UPDATE brands SET name = $2, updated_at = $3 WHERE id = $1 RETURNING *",
    brand_id.into_inner(),
    input.name,
    Utc::now(),
  )
  .fetch_optional(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let error_message = error.to_string();
      if error_message.contains("unique constraint") {
        let response = ErrorResponse {
          message: format!("Brand {} already exists", input.name),
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
        message: "Brand not found".to_string(),
      };

      return HttpResponse::NotFound().json(response);
    }
    Ok(Some(brand)) => {
      let response = UpdateBrandByIdResponse { brand };

      return HttpResponse::Ok().json(response);
    }
  }
}
