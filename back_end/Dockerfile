FROM rust:1.75.0-alpine3.19 as builder
WORKDIR /usr/src/axum_test
RUN apk add --no-cache musl-dev pkgconfig libressl-dev
RUN cargo install sqlx-cli
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs
COPY . .
RUN touch src/main.rs && cargo build --release

FROM alpine:3.19.0
COPY --from=builder /usr/src/axum_test/target/release/axum_test /usr/local/bin/axum_test
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY . .
RUN echo "DATABASE_URL=postgres://postgres:passwd@postgres" > .env
RUN echo "sqlx migrate run && axum_test" > launch.sh
CMD ["sh", "launch.sh"]
