# development
FROM rust:1.48.0 AS development

WORKDIR /app
# READMEとかこのファイルとかも突っ込んでるので./srcに絞っても良いかも
COPY . .
RUN rustup update
RUN rustup component add rustfmt --toolchain 1.48.0-x86_64-unknown-linux-gnu
RUN cargo install cargo-watch
