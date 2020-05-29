# -*- mode: dockerfile -*-

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:nightly as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/demo_rust_rocket

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/demo_rust_rocket*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 demo_rust_rocket

RUN adduser -D -s /bin/sh -u 1000 -G demo_rust_rocket demo_rust_rocket

WORKDIR /home/demo_rust_rocket/bin/

COPY --from=cargo-build /usr/src/demo_rust_rocket/target/x86_64-unknown-linux-musl/release/demo_rust_rocket .

RUN chown demo_rust_rocket:demo_rust_rocket demo_rust_rocket

USER demo_rust_rocket

CMD ["./demo_rust_rocket"]
