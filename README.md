# Car Rental API in Rust

Basic api for car rentals to study the Rust language

## Requirements

- The [Rust](https://www.rust-lang.org) programming language
- A [PostgreSQL](https://www.postgresql.org) database

## Instructions

1. Clone the project
1. Run the command `mv .env.example .env` to rename the `.env.example` file to `.env`
1. Fill in with the environment variables in the `.env` file
1. Run the database migrations with the command `cargo run --bin migrate`
1. Run the server with the command `cargo run --bin rust-car-rental-api`

### Environment Variables

- PORT: The port you want the server to run in. Example: `4000` (defaults to 8080)
- DATABASE_URL: A string with the information to connect to the database. Example: `postgres://postgres:postgres@localhost:5432/rust_car_rental`
- JWT_SECRET: A secret string to use to hash the jwt. Example: `uidgqw78dgqw78giodcqwuih`

## Routes

All application routes

### Brands

- `GET /brands` used to get the list of all brands
- `POST /brands` used to create a new brand
  ###### Data (json):
  - name `[string, required]`
    <br />
- `GET /brands/:id` used to get a brand by id
- `PUT /brands/:id` used to update a brand by id
  ###### Data (json):
  - name `[string, required]`
    <br />
- `DELETE /brands/:id` used to delete a brand by id

### Cars

- `GET /cars` used to get the list of all cars
- `POST /cars` used to create a new car
  ###### Data (json):
  - brand_id `[uuid, required]`
  - model `[string, required]`
  - horse_power `[int, required]`
  - torque_in_lb `[float, required]`
  - top_speed_in_km `[int, required]`
  - acceleration_speed_in_km `[float, required]`
  - weight_in_kg `[int, required]`
  - rental_price_daily_in_usd `[float, required]`
    <br />
- `GET /cars/:id` used to get a car by id
- `PUT /cars/:id` used to update a car by id
  ###### Data (json):
  - brand_id `[uuid, required]`
  - model `[string, required]`
  - horse_power `[int, required]`
  - torque_in_lb `[float, required]`
  - top_speed_in_km `[int, required]`
  - acceleration_speed_in_km `[float, required]`
  - weight_in_kg `[int, required]`
  - rental_price_daily_in_usd `[float, required]`
    <br />
- `DELETE /cars/:id` used to delete a car by id

### Auth

- `POST /auth/register` used to register new user
  ###### Data (json):
  - name `[string, required]`
  - email `[string, required]`
  - password `[string, required]`
    <br />
- `POST /auth/login` used to login
  ###### Data (json):
  - email `[string, required]`
  - password `[string, required]`
    <br />
- `GET /auth/me` used to get logged in user data `[requires auth]`

### Rentals

- `GET /rentals` used to get the list of all rentals
  ###### Filters (query params) (TODO):
  - car_id - if specified it will return only the rentals for this car_id `[optional]`
  - starts_at - if specified it will return only the rentals that have a starts_at greater than or equal to the specified date `[optional]`
  - ends_at - if specified it will return only the rentals that have an ends_at lesser than or equal to the specified date `[optional]`
    <br />
- `GET /rentals/mines` used to get the list of all rentals from logged user
  ###### Filters (query params) (TODO):
  - car_id - if specified it will return only the rentals for this car_id `[optional]`
  - starts_at - if specified it will return only the rentals that have a starts_at greater than or equal to the specified date `[optional]`
  - ends_at - if specified it will return only the rentals that have an ends_at lesser than or equal to the specified date `[optional]`
    <br />
- `POST /rentals` used to rent a car `[requires auth]`
  ###### Data (json)
  - car_id - The id of the car you want to rent `[required]`
  - starts_at - The start date for which you want to rent the car `[required]`
  - ends_at - The end date for which you want to rent the car `[required]`
    <br />
- `PATCH /rentals/:id/cancel` used to cancel a rental `[requires auth]`

[Insomnia import file](./insomnia.json)

## How to Authenticate

The `/auth/register` and the `/auth/login` endpoints return an `access_token`, send it as a Bearer token in the request, as the Authorization header, for example: `Bearer access_token_here`

## Tech Stack

- [Actix Web](https://actix.rs)
- [SQLx](https://github.com/launchbadge/sqlx)
- [serde](https://serde.rs)
- [Rust jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- [Rust dotenvy](https://github.com/allan2/dotenvy)
