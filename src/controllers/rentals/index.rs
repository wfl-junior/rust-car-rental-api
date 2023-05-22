use super::{RentalWithCarAndBrand, RentalsQueryParams};
use crate::{repositories, AppState, ErrorResponse};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct GetAllRentalsResponse {
  rentals: Vec<RentalWithCarAndBrand>,
}

#[get("/rentals")]
async fn get_all_rentals(
  request: HttpRequest,
  app_state: web::Data<AppState>,
) -> impl Responder {
  let query_params =
    web::Query::<RentalsQueryParams>::from_query(request.query_string());

  if let Err(error) = query_params {
    let response = ErrorResponse {
      message: error.to_string(),
    };

    return HttpResponse::BadRequest().json(response);
  }

  let params = query_params.unwrap();
  let filters = repositories::rental::GetAllRentalsFilters {
    user_id: None,
    car_id: params.car_id,
    starts_at: params.starts_at,
    ends_at: params.ends_at,
  };

  let result = repositories::rental::get_all(&app_state, filters).await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(rentals_with_car_and_brand) => {
      let response = GetAllRentalsResponse {
        rentals: rentals_with_car_and_brand,
      };

      return HttpResponse::Ok().json(response);
    }
  }
}
