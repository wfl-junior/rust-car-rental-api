use super::{get_by_id, Brand};
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct GetBrandByIdResponse {
  brand: Brand,
}

#[get("/brands/{id}")]
async fn get_brand_by_id(
  app_state: web::Data<AppState>,
  brand_id: web::Path<Uuid>,
) -> impl Responder {
  let result = get_by_id(&app_state.pool, brand_id.into_inner()).await;

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
    Ok(Some(brand)) => {
      let response = GetBrandByIdResponse { brand };
      return HttpResponse::Ok().json(response);
    }
  }
}
