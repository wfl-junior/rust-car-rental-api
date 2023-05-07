use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sqlx::postgres::PgConnection;
use sqlx::Connection;

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
}
