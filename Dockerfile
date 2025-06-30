# --- Build Stage ---
    FROM rust:1.76 as builder
    WORKDIR /usr/src/app
    COPY . .
    RUN cargo build --release
    
    # --- Runtime Stage ---
    FROM debian:buster-slim
    RUN useradd -m appuser
    WORKDIR /home/appuser
    COPY --from=builder /usr/src/app/target/release/api-server .
    RUN chown appuser:appuser api-server
    USER appuser
    
    EXPOSE 8080
    CMD ["./api-server"]