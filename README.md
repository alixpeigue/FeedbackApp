# REST API for workers report

## Requirements
 - Docker
 - Rust / Cargo
 - sqlx cli (install with `cargo install sqlx-cli`)

## To lauch

Launch the database with `docker compose up -d`

Launch the migrations with `sqlx migrate run`

Lauch the server with `cargo run`, the server tow listens on [localhost:3000](localhost:3000)
