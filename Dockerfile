FROM rust:latest as builder
WORKDIR /usr/src/mp3rename
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get upgrade && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mp3rename /usr/local/bin/mp3rename
CMD ["mp3rename"]
