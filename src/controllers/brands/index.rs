use super::Brand;
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct GetAllBrandsResponse {
  brands: Vec<Brand>,
}

#[get("/brands")]
async fn get_all_brands(app_state: web::Data<AppState>) -> impl Responder {
  let result = sqlx::query_as!(Brand, "SELECT * FROM brands;")
    .fetch_all(&app_state.pool)
    .await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(brands) => {
      let response = GetAllBrandsResponse { brands };
      return HttpResponse::Ok().json(response);
    }
  }
}
