FROM rust:1.63.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV APP_ENVIRONMENT production
ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]
