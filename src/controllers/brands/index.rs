use super::{BrandWithCars, BrandWithOptionCarQuery, Car};
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

fn append_car_to_brand(
  query: BrandWithOptionCarQuery,
  brand: &mut BrandWithCars,
) -> Result<(), ()> {
  let car = Car {
    id: query.car_id.ok_or(())?,
    created_at: query.car_created_at.ok_or(())?,
    updated_at: query.car_updated_at.ok_or(())?,
    brand_id: query.id,
    model: query.car_model.as_ref().ok_or(())?.to_string(),
    horse_power: query.car_horse_power.ok_or(())?,
    torque_in_lb: query.car_torque_in_lb.ok_or(())?,
    top_speed_in_km: query.car_top_speed_in_km.ok_or(())?,
    acceleration_speed_in_km: query.car_acceleration_speed_in_km.ok_or(())?,
    weight_in_kg: query.car_weight_in_kg.ok_or(())?,
    rental_price_daily_in_usd: query.car_rental_price_daily_in_usd.ok_or(())?,
  };

  brand.cars.push(car);
  return Ok(());
}

#[derive(Serialize)]
struct GetAllBrandsResponse {
  brands: Vec<BrandWithCars>,
}

#[get("/brands")]
async fn get_all_brands(app_state: web::Data<AppState>) -> impl Responder {
  let result = sqlx::query_as!(
    BrandWithOptionCarQuery,
    r#"
      SELECT
        brands.*,
        cars.id AS "car_id?",
        cars.created_at AS "car_created_at?",
        cars.updated_at AS "car_updated_at?",
        cars.model AS "car_model?",
        cars.horse_power AS "car_horse_power?",
        cars.torque_in_lb AS "car_torque_in_lb?",
        cars.top_speed_in_km AS "car_top_speed_in_km?",
        cars.acceleration_speed_in_km AS "car_acceleration_speed_in_km?",
        cars.weight_in_kg AS "car_weight_in_kg?",
        cars.rental_price_daily_in_usd AS "car_rental_price_daily_in_usd?"
      FROM brands
      LEFT OUTER JOIN cars ON brands.id = cars.brand_id
      ORDER BY brands.created_at ASC, cars.created_at ASC;
    "#
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
    Ok(brands_with_car_query) => {
      let mut brands_with_cars: Vec<BrandWithCars> = Vec::new();

      for query in brands_with_car_query {
        let existing_brand = brands_with_cars
          .iter_mut()
          .find(|brand| brand.id == query.id);

        let brand = match existing_brand {
          Some(brand) => brand,
          None => {
            let new_brand = BrandWithCars::from(&query);
            brands_with_cars.push(new_brand);
            brands_with_cars.last_mut().unwrap()
          }
        };

        append_car_to_brand(query, brand).unwrap_or(());
      }

      let response = GetAllBrandsResponse {
        brands: brands_with_cars,
      };

      return HttpResponse::Ok().json(response);
    }
  }
}
