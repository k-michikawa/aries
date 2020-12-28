# development
FROM rust:1.48.0 AS development

WORKDIR /app
ADD . .
RUN rustup update
RUN rustup component add rustfmt --toolchain 1.48.0-x86_64-unknown-linux-gnu
RUN cargo install cargo-watch

# production builder
FROM ekidd/rust-musl-builder:stable AS builder

# WORKDIR /usr/app
# RUN rustup target add x86_64-unknown-linux-musl
# RUN USER=root cargo new aries

# WORKDIR /usr/app/aries
# COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release

# COPY src ./src
# RUN cargo install --target x86_64-unknown-linux-musl --path .

WORKDIR /home/rust/src
RUN USER=root cargo new aries

WORKDIR /home/rust/src/aries
COPY src Cargo.toml Cargo.lock ./
RUN cargo build --release && \
    strip /home/rust/src/aries/target/x86_64-unknown-linux-musl/release/aries

# production runtime
FROM scratch AS production
COPY --from=builder /home/rust/src/aries/target/x86_64-unknown-linux-musl/release/aries /
ENTRYPOINT ["/aries"]
