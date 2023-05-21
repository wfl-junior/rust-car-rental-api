use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sqlx::{postgres::PgConnection, Connection};

#[actix_web::main]
async fn main() {
  dotenv().ok();
  let database_url = dotenv!("DATABASE_URL");
  let mut connection = PgConnection::connect(database_url).await.unwrap();

  sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")
    .execute(&mut connection)
    .await
    .unwrap();

  sqlx::query(
    "CREATE TABLE IF NOT EXISTS brands (
      id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
      created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      name varchar(255) NOT NULL UNIQUE
    )",
  )
  .execute(&mut connection)
  .await
  .unwrap();

  sqlx::query(
    "CREATE TABLE IF NOT EXISTS cars (
      id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
      created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      brand_id uuid NOT NULL,
      model varchar(255) NOT NULL UNIQUE,
      horse_power int4 NOT NULL,
      torque_in_lb float4 NOT NULL,
      top_speed_in_km int4 NOT NULL,
      acceleration_speed_in_km float4 NOT NULL,
      weight_in_kg int4 NOT NULL,
      rental_price_daily_in_usd float8 NOT NULL,

      CONSTRAINT fk_brand FOREIGN KEY(brand_id) REFERENCES brands(id)
    )",
  )
  .execute(&mut connection)
  .await
  .unwrap();

  sqlx::query(
    "CREATE TABLE IF NOT EXISTS users (
      id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
      created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      name varchar(255) NOT NULL,
      email varchar(255) NOT NULL UNIQUE,
      password varchar(255) NOT NULL
    )",
  )
  .execute(&mut connection)
  .await
  .unwrap();

  sqlx::query(
    "CREATE TABLE IF NOT EXISTS rentals (
      id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
      created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      user_id uuid NOT NULL,
      car_id uuid NOT NULL,
      starts_at timestamptz NOT NULL,
      ends_at timestamptz NOT NULL,
      canceled_at timestamptz,

      CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
      CONSTRAINT fk_car FOREIGN KEY(car_id) REFERENCES cars(id)
    )",
  )
  .execute(&mut connection)
  .await
  .unwrap();
}
