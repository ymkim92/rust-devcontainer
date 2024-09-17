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

RUN apt-get install -y sudo openssh-client 

# install just
# RUN wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
# RUN echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr jammy" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
# RUN apt-get update
# RUN apt-get install -y just 
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Create a non-root user with a home directory
# If you run your Dockerfile with a userid=1000 and groupid=1000, 
# the UID/GID updating will then update that user's UID and GID to your local ones.
ARG USERNAME=rustuser
ARG USER_UID=1000
ARG USER_GID=${USER_UID}
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} -s /bin/bash    

# Switch to the new user for Rust installation
USER ${USERNAME}

# Install Rust (Stable) using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set the environment path for Rust binaries
ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Install Rust components and tools
RUN rustup component add rustfmt clippy

# Clean up APT cache (still as rustuser, no need for root for this)
# RUN sudo apt-get clean && sudo rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspace

# Expose port 3000 (if necessary)
EXPOSE 3000
