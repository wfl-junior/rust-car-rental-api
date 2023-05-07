use super::{Brand, BrandInput};
use crate::{AppState, ErrorResponse};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct CreateBrandResponse {
  brand: Brand,
}

#[derive(Serialize)]
struct GetAllBrandsResponse {
  brands: Vec<Brand>,
}

#[post("/brands")]
async fn create_brand(
  app_state: web::Data<AppState>,
  input_json: web::Json<BrandInput>,
) -> impl Responder {
  let input = input_json.into_inner();
  let result = sqlx::query_as!(
    Brand,
    "INSERT INTO brands (name) VALUES ($1) RETURNING *;",
    input.name
  )
  .fetch_one(&app_state.pool)
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
    Ok(brand) => {
      let response = CreateBrandResponse { brand };
      return HttpResponse::Created().json(response);
    }
  };
}
