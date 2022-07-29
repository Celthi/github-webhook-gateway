FROM rust:1.58-buster

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["webhook_gateway"]
