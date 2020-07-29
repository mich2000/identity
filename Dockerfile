FROM rust:slim-stretch

COPY . ./identity

WORKDIR identity/identity_web

EXPOSE 8000

RUN rustup default nightly && cargo b --release && apt-get remove --purge -y $BUILD_PACKAGES $(apt-mark showauto) && rm -rf /var/lib/apt/lists/*

CMD ["cargo","r","--release"]