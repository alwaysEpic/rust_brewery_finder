# .devcontainer/Dockerfile

FROM rust:latest

# Install common Rust development tools
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rustup component add rustfmt clippy