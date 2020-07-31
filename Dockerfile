# Build stage
FROM rust:latest as cargoer

RUN apt-get update && rustup default nightly

COPY . .

WORKDIR $HOME/identity_web

RUN cargo build --release

# Final stage
FROM debian:stretch-slim

COPY --from=cargoer $HOME/identity_web/target/release/identity_web .

COPY $HOME/identity_web/.env .env

COPY $HOME/identity_web/Rocket.toml Rocket.toml

EXPOSE 8000

CMD ["./identity_web"]