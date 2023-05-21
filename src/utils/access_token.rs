use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{
  decode,
  encode,
  errors::Error,
  DecodingKey,
  EncodingKey,
  Header,
  TokenData,
  Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
  sub: Uuid,
  iat: i64,
  exp: i64,
}

pub fn generate_access_token(user_id: Uuid) -> Result<String, Error> {
  let now = Utc::now();

  let claims = JwtClaims {
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

fn decode_access_token(
  access_token: String,
) -> Result<TokenData<JwtClaims>, Error> {
  let secret = dotenv!("JWT_SECRET");

  return decode::<JwtClaims>(
    access_token.as_str(),
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  );
}

pub fn get_user_id_from_access_token(
  access_token: String,
) -> Result<Uuid, Error> {
  let token_data = decode_access_token(access_token)?;
  return Ok(Uuid::from(token_data.claims.sub));
}
