# Use the official Rust image as a builder stage
FROM rust:1.76-slim-buster as builder

# Create a new empty shell project
RUN USER=root cargo new --bin bikeseat
WORKDIR /bikeseat

# Copy the Cargo.toml and Cargo.lock files and download the Rust dependencies
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code of the Rust application
COPY ./src ./src

# Build the application for release
RUN rm ./target/release/deps/bikeseat*
RUN cargo build --release

# Use a minimal Debian buster image for the runtime stage
FROM debian:buster-slim

RUN mkdir out

# Install necessary packages and create a new user
RUN apt-get update \
    && apt-get install -y libssl1.1 ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m bikeseat

# USER bikeseat

# Copy the binary from the builder stage to the runtime stage
COPY --from=builder /bikeseat/target/release/bikeseat /usr/local/bin/bikeseat
COPY ./content content/
COPY ./static static/
COPY ./templates templates/

# Set the binary as the entrypoint of the container
ENTRYPOINT ["bikeseat"]
