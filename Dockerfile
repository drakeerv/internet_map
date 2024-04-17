FROM rust:1.72.2 as builder

WORKDIR /usr/src/map
COPY . .

RUN cargo install --path .
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/map/target/release/map /usr/local/bin/map/map
COPY --from=builder /usr/src/map/public /usr/local/bin/map/public

WORKDIR /usr/local/bin/map
CMD ["./map"]