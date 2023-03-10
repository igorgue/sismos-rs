FROM rust:latest as builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Will build all dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release

# Copy the rest
COPY . .
# Build (install) the actual binaries
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo install --path .

# Runtime image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev openssl ca-certificates

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY . /app

# create empty .env file
RUN touch .env

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/local/cargo/bin/sismos /app/sismos

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.
