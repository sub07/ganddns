FROM rust:1.85-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM alpine:3

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/ganddns /usr/local/bin/ganddns

WORKDIR /config
ENTRYPOINT ["ganddns"]
