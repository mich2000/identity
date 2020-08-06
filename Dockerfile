# Build stage
FROM rust:latest as cargoer

COPY . .

WORKDIR $HOME/identity_web

RUN rustup default nightly && cargo build --release && strip --strip-unneeded $HOME/identity_web/target/release/identity_web

# Final stage
FROM debian:stretch-slim

COPY --from=cargoer $HOME/identity_web/target/release .

EXPOSE 8000

CMD ["./identity_web"]