# Build stage
FROM rust:slim-stretch as cargoer

COPY identity_web .

COPY identity_dal .

COPY identity_service .

WORKDIR $HOME/identity_web

RUN rustup default nightly

RUN cargo build --release && strip target/release/identity_web

# Final stage
FROM debian:stretch-slim

COPY --from=cargoer $HOME/identity_web/target/release/identity_web .

COPY identity_web/.env .

COPY identity_web/Rocket.toml .

EXPOSE 8000

CMD ["./identity_web"]