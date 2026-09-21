FROM rust:1.88-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

RUN groupadd --gid 10001 app \
    && useradd --uid 10001 --gid app --no-create-home app \
    && mkdir /data \
    && chown app:app /data

COPY --from=builder /build/target/release/life-terminal /usr/local/bin/life-terminal

ENV TERM=xterm-256color
WORKDIR /data
USER app:app

ENTRYPOINT ["/usr/local/bin/life-terminal"]
