# Stage clang install 
FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 AS chef
# Equivalent to cd app. Docker creates the app folder if it does not exist.
WORKDIR /app
# Installs required deps for linking.
RUN apt update && apt install lld clang -y

# Stage planner
FROM chef AS planner
# Copy all from OUR working environment to our Docker image
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

# Stage builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build project deps not app
RUN cargo chef cook --release --recipe-path recipe.json
# If our deps stay the same, everything is cached up to this point
COPY . .
ENV SQLX_OFFLINE true
# Build the binary.
RUN cargo build --release --bin zero2prod

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
