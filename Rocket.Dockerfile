# Build stage
FROM rust:slim-stretch as cargoer

RUN apt-get update && apt-get -y install pkg-config libssl-dev

RUN rustup default nightly

COPY $HOME/identity_web ./identity_web

COPY $HOME/identity_dal ./identity_dal

COPY $HOME/identity_service ./identity_service

WORKDIR ./identity_web

RUN cargo build --release

# Final stage
FROM debian:stretch-slim

COPY --from=cargoer $HOME/identity_web/target/release/identity_web .

COPY --from=cargoer $HOME/identity_web/static .

RUN echo "/usr/local/lib64" > /etc/ld.so.conf.d/openssl.conf && ldconfig

COPY identity_web/.env .

COPY identity_web/Rocket.toml .

EXPOSE 8000

CMD ["./identity_web"]