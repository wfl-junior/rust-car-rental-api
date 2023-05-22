use crate::{
  controllers::rentals::{RentalWithCarAndBrand, RentalWithCarAndBrandQuery},
  AppState,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct GetAllRentalsFilters {
  pub user_id: Option<Uuid>,
  pub car_id: Option<Uuid>,
  pub starts_at: Option<DateTime<Utc>>,
  pub ends_at: Option<DateTime<Utc>>,
}

pub async fn get_all(
  app_state: &AppState,
  filters: GetAllRentalsFilters,
) -> sqlx::Result<Vec<RentalWithCarAndBrand>> {
  let mut query = "
    SELECT
      rentals.id AS id,
      rentals.created_at AS created_at,
      rentals.updated_at AS updated_at,
      rentals.car_id AS car_id,
      rentals.starts_at AS starts_at,
      rentals.ends_at AS ends_at,
      rentals.canceled_at AS canceled_at,
      cars.created_at AS car_created_at,
      cars.updated_at AS car_updated_at,
      cars.brand_id AS car_brand_id,
      cars.model AS car_model,
      cars.horse_power AS car_horse_power,
      cars.torque_in_lb AS car_torque_in_lb,
      cars.top_speed_in_km AS car_top_speed_in_km,
      cars.acceleration_speed_in_km AS car_acceleration_speed_in_km,
      cars.weight_in_kg AS car_weight_in_kg,
      cars.rental_price_daily_in_usd AS car_rental_price_daily_in_usd,
      brands.created_at AS car_brand_created_at,
      brands.updated_at AS car_brand_updated_at,
      brands.name AS car_brand_name
    FROM rentals
      INNER JOIN cars ON cars.id = rentals.car_id
      INNER JOIN brands ON brands.id = cars.brand_id
  "
  .to_string();

  let mut uuid_params: Vec<Uuid> = Vec::new();
  let mut datetime_params: Vec<DateTime<Utc>> = Vec::new();

  if let Some(user_id) = filters.user_id {
    query.push_str(" WHERE rentals.user_id = $1");
    uuid_params.push(user_id);
  }

  if let Some(car_id) = filters.car_id {
    if uuid_params.is_empty() {
      query.push_str(" WHERE");
    } else {
      query.push_str(" AND");
    }

    query.push_str(" rentals.car_id = $");
    query.push_str(&(uuid_params.len() + 1).to_string());
    uuid_params.push(car_id);
  }

  if let Some(starts_at) = filters.starts_at {
    if uuid_params.is_empty() {
      query.push_str(" WHERE");
    } else {
      query.push_str(" AND");
    }

    query.push_str(" rentals.starts_at >= $");
    query.push_str(&(uuid_params.len() + 1).to_string());
    datetime_params.push(starts_at);
  }

  if let Some(ends_at) = filters.ends_at {
    if uuid_params.is_empty() && datetime_params.is_empty() {
      query.push_str(" WHERE");
    } else {
      query.push_str(" AND");
    }

    query.push_str(" rentals.ends_at <= $");
    let len = uuid_params.len() + datetime_params.len() + 1;
    query.push_str(&len.to_string());
    datetime_params.push(ends_at);
  }

  query.push_str(" ORDER BY rentals.created_at ASC;");

  let mut query_builder =
    sqlx::query_as::<_, RentalWithCarAndBrandQuery>(&query);

  for param in uuid_params {
    query_builder = query_builder.bind(param);
  }

  for param in datetime_params {
    query_builder = query_builder.bind(param);
  }

  let result = query_builder.fetch_all(&app_state.pool).await?;

  return Ok(
    result
      .into_iter()
      .map(RentalWithCarAndBrand::from)
      .collect(),
  );
}
