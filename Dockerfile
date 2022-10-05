FROM rust:1.64.0 as builder
WORKDIR /usr/src/ip-extract
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt update && rm -rf /var/lib/apt/lists/*
    && ln -s /usr/local/lib64/libssl.so.1.1 /usr/lib64/libssl.so.1.1
    && ln -s /usr/local/lib64/libcrypto.so.1.1 /usr/lib64/libcrypto.so.1.1
COPY --from=builder /usr/local/cargo/bin/ip-extract /usr/local/bin/ip-extract
CMD ["ip-extract"]