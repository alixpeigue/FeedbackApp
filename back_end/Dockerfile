FROM rust:1.75.0-alpine3.19 as builder
WORKDIR /usr/src/axum_test
RUN apk add --no-cache musl-dev pkgconfig libressl-dev
COPY . .
RUN echo "DATABASE_URL=postgres://postgres:passwd@postgres" > .env
RUN cargo install sqlx-cli
RUN sqlx migrate run
RUN cargo install --path .

FROM alpine:3.19.0
COPY --from=builder /usr/local/cargo/bin/axum_test /usr/local/bin/axum_test
CMD ["axum_test"]
