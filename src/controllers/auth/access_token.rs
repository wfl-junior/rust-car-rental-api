use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: Uuid,
  iat: i64,
  exp: i64,
}

pub fn generate_access_token(user_id: Uuid) -> Result<String, Error> {
  let now = Utc::now();

  let claims = Claims {
    sub: user_id,
    iat: now.timestamp(),
    exp: (now + Duration::days(1)).timestamp(),
  };

  let secret = dotenv!("JWT_SECRET");

  return encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  );
}
