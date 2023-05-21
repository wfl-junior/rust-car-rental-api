use super::User;
use crate::{middleware::auth::AuthMiddleware, AppState, ErrorResponse};
use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct MeResponse {
  user: User,
}

#[get("/auth/me")]
async fn get_me(
  request: HttpRequest,
  app_state: web::Data<AppState>,
  _: AuthMiddleware,
) -> impl Responder {
  let extensions = request.extensions();
  let user_id = extensions.get::<Uuid>().unwrap();

  let result = sqlx::query_as!(
    User,
    "SELECT id, created_at, updated_at, name, email FROM users WHERE id = $1",
    user_id,
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
      return HttpResponse::NotFound().into();
    }
    Ok(Some(user)) => {
      let response = MeResponse { user };

      return HttpResponse::Ok().json(response);
    }
  };
}
