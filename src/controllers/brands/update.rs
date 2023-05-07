use super::{get_by_id, Brand, BrandInput};
use crate::{AppState, ErrorResponse};
use actix_web::{put, web, HttpResponse, Responder};
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
  let id = brand_id.into_inner();
  let result = get_by_id(&app_state.pool, id).await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(None) => {
      let response = ErrorResponse {
        message: "Brand not found".to_string(),
      };

      return HttpResponse::NotFound().json(response);
    }
    Ok(Some(mut brand)) => {
      let input = input_json.into_inner();
      brand.name = input.name;

      let result = sqlx::query("UPDATE brands SET name = $2 WHERE id = $1")
        .bind(id)
        .bind(&brand.name)
        .execute(&app_state.pool)
        .await;

      match result {
        Err(error) => {
          let error_message = error.to_string();
          if error_message.contains("unique constraint") {
            let response = ErrorResponse {
              message: format!("Brand {} already exists", brand.name),
            };

            return HttpResponse::Conflict().json(response);
          }

          let response = ErrorResponse {
            message: error_message,
          };

          return HttpResponse::InternalServerError().json(response);
        }
        Ok(_) => {
          let response = UpdateBrandByIdResponse { brand };

          return HttpResponse::Ok().json(response);
        }
      }
    }
  }
}
