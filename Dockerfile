# --- Build Stage ---
FROM rust:1.76 as builder
WORKDIR /usr/src/app

# Copy Cargo.toml first for better caching
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy the actual source code
COPY src ./src
RUN cargo build --release

# --- Runtime Stage ---
FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd -m appuser
WORKDIR /home/appuser
COPY --from=builder /usr/src/app/target/release/api-server .
RUN chown appuser:appuser api-server
USER appuser

EXPOSE 8080
CMD ["./api-server"]