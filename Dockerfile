FROM rust:alpine3.17 as builder

WORKDIR /app

RUN apk add musl-dev

RUN cargo init

COPY Cargo.toml Cargo.lock ./
COPY ./src src
RUN cargo build
RUN cargo clean -p dummyhttp

RUN cargo install --path . --target=x86_64-unknown-linux-musl

# copy the binaries. dummyhttp is at /usr/local/bin
FROM alpine:3.17
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin
