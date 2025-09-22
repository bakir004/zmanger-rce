FROM rust:1.82-slim

RUN apt-get update && apt-get install -y podman && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release
RUN mkdir -p /app/code

# Pre-pull the GCC image during build to avoid runtime delays
# RUN podman pull docker.io/library/gcc:latest

EXPOSE 8080
CMD ["cargo", "run", "--release"]