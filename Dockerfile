FROM rust:latest as builder

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/server ./server
CMD ["server"]
