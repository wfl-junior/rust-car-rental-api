use super::Rental;
use crate::{middleware::auth::AuthMiddleware, AppState, ErrorResponse};
use actix_web::{
  patch,
  web,
  HttpMessage,
  HttpRequest,
  HttpResponse,
  Responder,
};
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct CancelRentalResponse {
  rental: Rental,
}

async fn handle_cancel_rental(
  app_state: web::Data<AppState>,
  mut rental: Rental,
) -> HttpResponse {
  let now = Utc::now();
  let result = sqlx::query_as!(
    Rental,
    "UPDATE rentals SET canceled_at = $2 WHERE id = $1;",
    rental.id,
    now
  )
  .execute(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(_) => {
      rental.canceled_at = Some(now);
      let response = CancelRentalResponse { rental };
      return HttpResponse::Ok().json(response);
    }
  };
}

#[patch("/rentals/{id}")]
async fn cancel_rental(
  request: HttpRequest,
  app_state: web::Data<AppState>,
  rental_id: web::Path<Uuid>,
  _: AuthMiddleware,
) -> impl Responder {
  let extensions = request.extensions();
  let user_id = extensions.get::<Uuid>().unwrap();

  let result = sqlx::query_as!(
    Rental,
    "SELECT * FROM rentals WHERE id = $1",
    rental_id.into_inner()
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
        message: String::from("Rental not found"),
      };

      return HttpResponse::NotFound().json(response);
    }
    Ok(Some(rental)) => {
      // return forbidden if the user trying to cancel the rental is not the owner
      if rental.user_id != user_id.to_owned() {
        let response = ErrorResponse {
          message: String::from("You must be the owner to cancel the rental"),
        };

        return HttpResponse::Forbidden().json(response);
      }

      // return bad request if the rental is already canceled
      if rental.canceled_at.is_some() {
        let response = ErrorResponse {
          message: String::from("This rental has already been canceled"),
        };

        return HttpResponse::Forbidden().json(response);
      }

      // return bad request if the rental start date has passed
      if rental.starts_at < Utc::now() {
        let response = ErrorResponse {
          message: String::from(
            "You cannot cancel a rental that has already started",
          ),
        };

        return HttpResponse::Forbidden().json(response);
      }

      return handle_cancel_rental(app_state, rental).await;
    }
  }
}
