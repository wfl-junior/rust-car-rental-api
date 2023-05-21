use super::{RegisterInput, User};
use crate::{
  utils::access_token::generate_access_token,
  AppState,
  ErrorResponse,
};
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::hash;
use serde::Serialize;

#[derive(Serialize)]
struct RegisterResponse {
  access_token: String,
  user: User,
}

fn handle_generate_access_token(user: User) -> HttpResponse {
  match generate_access_token(user.id) {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(access_token) => {
      let response = RegisterResponse { access_token, user };

      return HttpResponse::Created().json(response);
    }
  }
}

#[post("/auth/register")]
async fn register(
  app_state: web::Data<AppState>,
  input_json: web::Json<RegisterInput>,
) -> impl Responder {
  let input = input_json.into_inner();
  let hashed_password = hash(input.password, 10);

  if let Err(error) = hashed_password {
    let response = ErrorResponse {
      message: error.to_string(),
    };

    return HttpResponse::InternalServerError().json(response);
  }

  let result = sqlx::query_as!(
    User,
    "
      INSERT INTO users
        (name, email, password)
      VALUES
        ($1, $2, $3)
      RETURNING id, created_at, updated_at, name, email;",
    input.name,
    input.email,
    hashed_password.unwrap()
  )
  .fetch_one(&app_state.pool)
  .await;

  match result {
    Err(error) => {
      let error_message = error.to_string();
      if error_message.contains("unique constraint") {
        let response = ErrorResponse {
          message: format!("E-mail {} already exists", input.email),
        };

        return HttpResponse::Conflict().json(response);
      }

      let response = ErrorResponse {
        message: error_message,
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(user) => {
      return handle_generate_access_token(user);
    }
  };
}
