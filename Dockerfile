FROM rust:slim-stretch as buidler

ADD . .

WORKDIR $HOME/identity_web

EXPOSE 8000

RUN rustup default nightly && cargo install --path .

FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/identity_web /usr/local/bin/identity_web

CMD ["identity_web"]