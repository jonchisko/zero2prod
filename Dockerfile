# Stage builder
FROM rust:1.65.0 AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
# Build the binary.
RUN cargo build --release

# Runtime stage
# Slim debian OS
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - dynamically linked by some of our deps 
# ca-certificates -> needed for TLS certificates for HTTPS
RUN apt-get update -y && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need configuration file at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production

# When docker run is executed, launch the binary.
ENTRYPOINT [ "./zero2prod" ]
