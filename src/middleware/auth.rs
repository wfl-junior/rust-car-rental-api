use crate::utils::access_token::get_user_id_from_access_token;
use actix_web::{
  dev::Payload,
  error::ErrorUnauthorized,
  http,
  Error as ActixWebError,
  FromRequest,
  HttpMessage,
  HttpRequest,
};
use core::fmt;
use serde::Serialize;
use serde_json;
use std::future::{ready, Ready};
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct ErrorResponse {
  message: String,
}

impl fmt::Display for ErrorResponse {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    return write!(formatter, "{}", serde_json::to_string(&self).unwrap());
  }
}

pub struct AuthMiddleware {
  pub user_id: Uuid,
}

impl FromRequest for AuthMiddleware {
  type Error = ActixWebError;
  type Future = Ready<Result<Self, Self::Error>>;
  fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
    let access_token =
      request
        .headers()
        .get(http::header::AUTHORIZATION)
        .map(|authorization| {
          authorization.to_str().unwrap().split_at(7).1.to_string()
        });

    if access_token.is_none() {
      let response = ErrorResponse {
        message: String::from(
          "Provide your bearer access token in Authorization header",
        ),
      };

      return ready(Err(ErrorUnauthorized(response)));
    }

    match get_user_id_from_access_token(access_token.unwrap()) {
      Err(error) => {
        let response = ErrorResponse {
          message: error.to_string(),
        };

        return ready(Err(ErrorUnauthorized(response)));
      }
      Ok(user_id) => {
        request.extensions_mut().insert::<Uuid>(user_id);
        return ready(Ok(AuthMiddleware { user_id }));
      }
    }
  }
}
