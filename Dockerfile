# Build stage
FROM rust:latest as cargoer

RUN apt-get update && apt-get install musl-tools -y && rustup target add x86_64-unknown-linux-musl

COPY . .

WORKDIR $HOME/identity_web

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Final stage
FROM alpine:latest

RUN addgroup -g 1000 identity_web

RUN adduser -D -s /bin/sh -u 1000 -G identity_web identity_web

COPY --from=cargo-build $HOME/identity_web/target/x86_64-unknown-linux-musl/release/identity_web .

RUN chown identity_web:identity_web identity_web

USER identity_web

CMD ["./identity_web"]