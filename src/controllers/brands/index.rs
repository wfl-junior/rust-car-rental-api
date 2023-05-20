use super::{BrandWithCars, BrandWithOptionCarQuery, Car};
use crate::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

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

        let car = query.car_id.and_then(|car_id| {
          query.car_created_at.and_then(|created_at| {
            query.car_updated_at.and_then(|updated_at| {
              query.car_model.as_ref().and_then(|model| {
                query.car_horse_power.and_then(|horse_power| {
                  query.car_torque_in_lb.and_then(|torque_in_lb| {
                    query.car_top_speed_in_km.and_then(|top_speed_in_km| {
                      query
                        .car_acceleration_speed_in_km
                        .and_then(|acceleration_speed_in_km| {
                          query.car_weight_in_kg.and_then(|weight_in_kg| {
                            query
                              .car_rental_price_daily_in_usd
                              .map(|rental_price_daily_in_usd| Car {
                                id: car_id,
                                created_at,
                                updated_at,
                                brand_id: query.id,
                                model: model.to_string(),
                                horse_power,
                                torque_in_lb,
                                top_speed_in_km,
                                acceleration_speed_in_km,
                                weight_in_kg,
                                rental_price_daily_in_usd,
                              })
                          })
                        })
                    })
                  })
                })
              })
            })
          })
        });

        if let Some(car) = car {
          brand.cars.push(car);
        }
      }

      let response = GetAllBrandsResponse {
        brands: brands_with_cars,
      };

      return HttpResponse::Ok().json(response);
    }
  }
}
