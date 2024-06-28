FROM rust:latest as builder

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/server ./server
COPY --from=builder ./static ./static
COPY --from=builder ./pages/ ./pages
CMD ["server"]
