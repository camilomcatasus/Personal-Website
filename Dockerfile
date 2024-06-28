FROM rust:bookworm as builder

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/server /app/server
COPY --from=builder ./static /app/static
COPY --from=builder ./pages/ /app/pages
CMD ["/app/server"]
