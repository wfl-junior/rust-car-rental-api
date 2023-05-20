use super::{CarWithBrand, CarWithBrandQuery};
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct GetAllCarsResponse {
  cars: Vec<CarWithBrand>,
}

#[get("/cars")]
async fn get_all_cars(app_state: web::Data<AppState>) -> impl Responder {
  let result = sqlx::query_as!(
    CarWithBrandQuery,
    "
      SELECT
        cars.*,
        brands.created_at AS brand_created_at,
        brands.updated_at AS brand_updated_at,
        brands.name AS brand_name
      FROM cars INNER JOIN brands ON cars.brand_id = brands.id
      ORDER BY cars.created_at ASC;
    "
  )
  .fetch_all(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(cars_with_brand_query) => {
      let cars_with_brand: Vec<CarWithBrand> = cars_with_brand_query
        .into_iter()
        .map(CarWithBrand::from)
        .collect();

      let response = GetAllCarsResponse {
        cars: cars_with_brand,
      };

      return HttpResponse::Ok().json(response);
    }
  }
}
