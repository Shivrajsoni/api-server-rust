# --- Build Stage ---
FROM rust:1.76 as builder
WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Build the application
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