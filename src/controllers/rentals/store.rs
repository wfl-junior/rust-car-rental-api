use super::Rental;
use crate::{middleware::auth::AuthMiddleware, AppState, ErrorResponse};
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateRentalInput {
  car_id: Uuid,
  starts_at: DateTime<Utc>,
  ends_at: DateTime<Utc>,
}

struct HasOverlappingResult {
  count: i64,
}

async fn has_overlapping(
  app_state: &web::Data<AppState>,
  input: &CreateRentalInput,
) -> sqlx::Result<bool> {
  let result = sqlx::query_as!(
    HasOverlappingResult,
    r#"
      SELECT COUNT(id) AS "count!"
      FROM rentals
      WHERE 
        car_id = $1
        AND canceled_at IS NULL
        AND (starts_at < $3 AND ends_at > $2);
    "#,
    input.car_id,
    input.starts_at,
    input.ends_at
  )
  .fetch_one(&app_state.pool)
  .await?;

  return Ok(result.count > 0);
}

#[derive(Serialize)]
struct CreateRentalResponse {
  rental: Rental,
}

async fn handle_create_rental(
  app_state: web::Data<AppState>,
  user_id: &Uuid,
  input: CreateRentalInput,
) -> HttpResponse {
  let result = sqlx::query_as!(
    Rental,
    "
      INSERT INTO rentals (user_id, car_id, starts_at, ends_at)
      VALUES ($1, $2, $3, $4)
      RETURNING *;
    ",
    user_id,
    input.car_id,
    input.starts_at,
    input.ends_at,
  )
  .fetch_one(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(rental) => {
      let response = CreateRentalResponse { rental };
      return HttpResponse::Ok().json(response);
    }
  };
}

#[post("/rentals")]
async fn create_rental(
  request: HttpRequest,
  app_state: web::Data<AppState>,
  input_json: web::Json<CreateRentalInput>,
  _: AuthMiddleware,
) -> impl Responder {
  let extensions = request.extensions();
  let user_id = extensions.get::<Uuid>().unwrap();
  let input = input_json.into_inner();
  let now = Utc::now();

  // return bad request if the period is in the past
  if input.starts_at < now || input.ends_at < now {
    let response = ErrorResponse {
      message: String::from("The period cannot be in the past"),
    };

    return HttpResponse::BadRequest().json(response);
  }

  // return bad request if the period end date is before start date
  if input.ends_at < input.starts_at {
    let response = ErrorResponse {
      message: String::from(
        "The period end date cannot be before the period start date",
      ),
    };

    return HttpResponse::BadRequest().json(response);
  }

  // return bad request if the period is less than 1 hour
  if input
    .ends_at
    .signed_duration_since(input.starts_at)
    .num_hours()
    < 1
  {
    let response = ErrorResponse {
      message: String::from("The period must be at least 1 hour"),
    };

    return HttpResponse::BadRequest().json(response);
  }

  match has_overlapping(&app_state, &input).await {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(true) => {
      let response = ErrorResponse {
        message: String::from(
          "This car is already rented during the specified period",
        ),
      };

      return HttpResponse::BadRequest().json(response);
    }
    Ok(false) => {
      return handle_create_rental(app_state, user_id, input).await;
    }
  }
}
