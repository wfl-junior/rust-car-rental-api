use super::{Brand, BrandInput};
use crate::{AppState, ErrorResponse};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

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
  let now = Utc::now();
  let input = input_json.into_inner();
  let brand = Brand {
    id: Uuid::new_v4(),
    name: input.name,
    created_at: now.clone(),
    updated_at: now.clone(),
  };

  let result =
    sqlx::query("INSERT INTO brands (id, name, created_at, updated_at) VALUES ($1, $2, $3, $4);")
      .bind(&brand.id)
      .bind(&brand.name)
      .bind(&brand.created_at)
      .bind(&brand.updated_at)
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
      let response = CreateBrandResponse { brand };
      return HttpResponse::Created().json(response);
    }
  };
}
