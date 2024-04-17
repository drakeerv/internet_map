FROM rust:1.72.2 as builder

WORKDIR /usr/src/internet_map
COPY . .

RUN cargo install --path .
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/internet_map/target/release/internet_map /usr/local/bin/internet_map/internet_map
COPY --from=builder /usr/src/internet_map/public /usr/local/bin/internet_map/public

WORKDIR /usr/local/bin/map
CMD ["./map"]

# build: docker build -t map .