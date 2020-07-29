FROM rust:slim-stretch

COPY . ./identity

WORKDIR identity/identity_web

EXPOSE 8000

RUN rustup default nightly && cargo b --release

CMD ["cargo","r","--release"]