# Latest rust.
FROM rust:1.63.0

# Equivalent to cd app. Docker creates the app folder if it does not exist.
WORKDIR /app

# Installs required deps for linking.
RUN apt update && apt install lld clang -y

# Copy all from OUR working environment to our Docker image
COPY . .

# Build the binary.
RUN cargo build --release

# When docker run is executed, launch the binary.
ENTRYPOINT [ "./target/release/zero2prod" ]
