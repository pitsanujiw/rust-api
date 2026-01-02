# ---------- build stage ----------
FROM rust:1.75-bookworm AS builder
WORKDIR /app

# cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ---------- runtime stage ----------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# copy binary
COPY --from=builder /app/target/release/rust-crud /app/app

# expose port
EXPOSE 3000

ENV RUST_LOG=info,tower_http::trace=info
ENV APP_ENV=prod

CMD ["/app/app"]
