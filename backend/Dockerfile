FROM rust:1.85-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    g++ \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire project for building
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:12-slim

# Install OpenSSL and CA certificates
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# set environment variables
ENV RUST_ENV=production
ENV RUST_BACKTRACE=1

COPY .env.production .env.production

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Expose the port
EXPOSE 8000

# Run the binary
CMD ["./backend"]