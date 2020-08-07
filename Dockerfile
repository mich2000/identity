# Build stage
FROM rust:latest as cargoer

COPY . .

WORKDIR $HOME/identity_web

RUN rustup default nightly && cargo build --release

# Final stage
FROM debian:stretch-slim

ENV USER=IDENTITY_USER

COPY --from=cargoer $HOME/identity_web/target/release/identity_web .

COPY identity_web/.env .

COPY identity_web/Rocket.toml .

RUN groupadd ${USER} && useradd -g ${USER} ${USER} && RUN chown -R ${USER}:${USER} identity_web

USER ${USER}

EXPOSE 8000

CMD ["./identity_web"]