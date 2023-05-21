use super::{CarWithBrand, CarWithBrandQuery};
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct GetCarByIdResponse {
  car: CarWithBrand,
}

#[get("/cars/{id}")]
async fn get_car_by_id(
  app_state: web::Data<AppState>,
  car_id: web::Path<Uuid>,
) -> impl Responder {
  let result = sqlx::query_as!(
    CarWithBrandQuery,
    "
      SELECT
        cars.*,
        brands.created_at AS brand_created_at,
        brands.updated_at AS brand_updated_at,
        brands.name AS brand_name
      FROM cars INNER JOIN brands ON brands.id = cars.brand_id
      WHERE cars.id = $1
      LIMIT 1;
    ",
    car_id.into_inner()
  )
  .fetch_optional(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(None) => {
      let response = ErrorResponse {
        message: "Car not found".to_string(),
      };

      return HttpResponse::NotFound().json(response);
    }
    Ok(Some(car_with_brand_query)) => {
      let response = GetCarByIdResponse {
        car: CarWithBrand::from(car_with_brand_query),
      };

      return HttpResponse::Ok().json(response);
    }
  }
}
