FROM rust:1.85.0-slim-bullseye

WORKDIR /usr/src/app

RUN cargo install cargo-watch

RUN apt-get update && apt-get install -y pkg-config libssl-dev

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]
