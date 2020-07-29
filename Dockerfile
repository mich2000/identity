FROM rust:slim-stretch

ADD . .

WORKDIR $HOME/identity_web

EXPOSE 8000

RUN rustup default nightly && \
cargo build --release && \
strip target/release/identity_web && \
apt-get autoremove && \
rm -rf /var/lib/apt/lists/*

CMD ["cargo","r","--release"]