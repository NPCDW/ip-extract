FROM rust:1.64.0 as builder
WORKDIR /usr/src/ip-extract
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ip-extract /usr/local/bin/ip-extract
CMD ["ip-extract"]