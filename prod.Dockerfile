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
COPY src ./src
COPY proto ./proto
COPY Cargo.toml Cargo.lock build.rs ./
RUN cargo build --release && \
    strip /home/rust/src/aries/target/x86_64-unknown-linux-musl/release/aries

# production runtime
FROM scratch AS production
COPY --from=builder /home/rust/src/aries/target/x86_64-unknown-linux-musl/release/aries /
ENTRYPOINT ["/aries"]