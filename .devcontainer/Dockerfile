# Use Ubuntu 22.04 as the base image
FROM ubuntu:22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install required dependencies as root
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    git \
    ca-certificates \
    lldb \
    python3 \
    python3-pip \
    --no-install-recommends

RUN apt-get install -y sudo
# Create a non-root user with a home directory
RUN useradd -ms /bin/bash rustuser

# Switch to the new user for Rust installation
USER rustuser

# Install Rust (Stable) using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set the environment path for Rust binaries
ENV PATH=/home/rustuser/.cargo/bin:$PATH

# Install Rust components and tools
RUN rustup component add rustfmt clippy

# Clean up APT cache (still as rustuser, no need for root for this)
# RUN sudo apt-get clean && sudo rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspace

# Expose port 3000 (if necessary)
EXPOSE 3000
