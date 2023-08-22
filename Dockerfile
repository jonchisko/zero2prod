# Latest rust.
FROM rust:1.65.0

# Equivalent to cd app. Docker creates the app folder if it does not exist.
WORKDIR /app

# Installs required deps for linking.
RUN apt update && apt install lld clang -y

# Copy all from OUR working environment to our Docker image
COPY . .

ENV SQLX_OFFLINE true

# Build the binary.
RUN cargo build --release

ENV APP_ENVIRONMENT production

# When docker run is executed, launch the binary.
ENTRYPOINT [ "./target/release/zero2prod" ]
