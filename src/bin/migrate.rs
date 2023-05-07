use dotenv::dotenv;
use sqlx::postgres::PgConnection;
use sqlx::Connection;

#[actix_web::main]
async fn main() {
  dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let mut connection = PgConnection::connect(&database_url).await.unwrap();

  sqlx::query(
    "CREATE TABLE IF NOT EXISTS brands (
      id uuid PRIMARY KEY,
      created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
      name varchar(255) NOT NULL UNIQUE
    )",
  )
  .execute(&mut connection)
  .await
  .unwrap();
}
