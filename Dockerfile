FROM rust:latest AS build

RUN mkdir /usr/src/ip-extract
WORKDIR /usr/src/ip-extract
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo build --release


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates
COPY --from=build /usr/src/ip-extract/target/release/ip-extract /usr/local/bin/ip-extract
CMD ip-extract