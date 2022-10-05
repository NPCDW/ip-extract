FROM rust:1.64.0

WORKDIR /usr/src/ip-extract
COPY . .

RUN cargo install --path .

CMD ["ip-extract"]