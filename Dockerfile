# -*- mode: dockerfile -*-

# This Dockerfile is optional to use with this demo; we provide it for 
# developers who like to use Docker in order to build containers.
#
# To use this:
#
#      docker login
#      docker build . -t demo_rust_rocket     

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM alpine:latest as builder

# Update the system as needed
RUN apk update

# Install typical system packages we use for building our app.
RUN apk add binutils
RUN apk add build-base
RUN apk add ca-certificates
RUN apk add curl
RUN apk add file
RUN apk add g++
RUN apk add gcc
RUN apk add libressl-dev
RUN apk add make
RUN apk add patch
RUN apk add postgresql
RUN apk add rust
RUN apk add sqlite

# Install Rust with typical settings and as a typical user.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Rust nightly and target for musl 
RUN . ~/.cargo/env && rustup toolchain install nightly 
RUN . ~/.cargo/env && rustup default nightly
RUN . ~/.cargo/env && rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/demo_rust_rocket

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

#TODO move up
#RUN . ~/.cargo/env && RUSTFLAGS="-C linker=musl-gcc -C target-feature=-crt-static" cargo build --release --target=x86_64-unknown-linux-musl
RUN . ~/.cargo/env && RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/demo_rust_rocket*

COPY . .

#RUN . ~/.cargo/env RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 demo_rust_rocket

RUN adduser -D -s /bin/sh -u 1000 -G demo_rust_rocket demo_rust_rocket

WORKDIR /home/demo_rust_rocket/bin/

COPY --from=builder /usr/src/demo_rust_rocket/target/x86_64-unknown-linux-musl/release/demo_rust_rocket .

RUN chown demo_rust_rocket:demo_rust_rocket demo_rust_rocket

USER demo_rust_rocket

CMD ["./demo_rust_rocket"]
