# Build stage for Rust backend
FROM rust:1.75-slim AS backend-builder

WORKDIR /app

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy backend files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./

# Build backend
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    libssl3 \
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/reading-notes-backend /app/
COPY --from=backend-builder /app/migrations /app/migrations

# Copy frontend build
COPY frontend/dist /app/static

# Change ownership
RUN chown -R appuser:appuser /app

USER appuser

# Set environment variables
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -q --tries=1 -O - http://localhost:8080/health || exit 1

# Start the application
CMD ["./reading-notes-backend"]