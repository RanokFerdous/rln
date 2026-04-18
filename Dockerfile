# --- Build Stage ---
FROM rust:1.75-slim-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/rln
COPY . .

# Build the application
# Using --release for production-ready binary
RUN cargo build --release

# --- Runtime Stage ---
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    sqlite3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create data directory for SQLite and config
RUN mkdir -p /data
VOLUME /data

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rln/target/release/rln /usr/local/bin/rln

# Set environment variables
ENV RLN_DATA_DIR=/data

# Entrypoint
ENTRYPOINT ["rln"]
CMD ["--dashboard"]
