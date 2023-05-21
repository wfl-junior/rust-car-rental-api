use super::{
  access_token::generate_access_token, LoginInput, User, UserWithPassword,
};
use crate::{AppState, ErrorResponse};
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::verify;
use serde::Serialize;

#[derive(Serialize)]
struct LoginResponse {
  access_token: String,
  user: User,
}

fn handle_generate_access_token(
  user_with_password: UserWithPassword,
) -> HttpResponse {
  match generate_access_token(user_with_password.id) {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(access_token) => {
      let response = LoginResponse {
        access_token,
        user: User {
          id: user_with_password.id,
          created_at: user_with_password.created_at,
          updated_at: user_with_password.updated_at,
          name: user_with_password.name,
          email: user_with_password.email,
        },
      };

      return HttpResponse::Created().json(response);
    }
  }
}

fn handle_verify_password(
  password: String,
  user_with_password: UserWithPassword,
) -> HttpResponse {
  match verify(password, &user_with_password.password) {
    Err(error) => {
      let response = ErrorResponse {
        message: error.to_string(),
      };

      return HttpResponse::InternalServerError().json(response);
    }
    Ok(false) => {
      let response = ErrorResponse {
        message: String::from("Invalid credentials"),
      };

      return HttpResponse::BadRequest().json(response);
    }
    Ok(true) => {
      return handle_generate_access_token(user_with_password);
    }
  }
}

#[post("/auth/login")]
async fn login(
  app_state: web::Data<AppState>,
  input_json: web::Json<LoginInput>,
) -> impl Responder {
  let input = input_json.into_inner();

  let result = sqlx::query_as!(
    UserWithPassword,
    "SELECT * FROM users WHERE email = $1",
    input.email,
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
        message: String::from("Invalid credentials"),
      };

      return HttpResponse::BadRequest().json(response);
    }
    Ok(Some(user_with_password)) => {
      return handle_verify_password(input.password, user_with_password);
    }
  };
}
