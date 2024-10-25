Certainly! Here’s a guide to creating a development container for embedded Rust development based on Ubuntu 22.04. This setup includes the Rust toolchain, ARM cross-compilation targets, and debugging tools to get you started with embedded Rust development.

### 1. **Folder Structure**
   - **`Dockerfile`**: Defines the container environment.
   - **`devcontainer.json`**: Configures the development container’s VS Code extensions and settings.

### 2. **Dockerfile**
Create a `Dockerfile` in your project root. This Dockerfile will:
   - Use Ubuntu 22.04 as the base image.
   - Install essential tools and dependencies.
   - Install Rust, set up cross-compilation targets, and configure the environment.

```dockerfile
# Base image
FROM ubuntu:22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=$CARGO_HOME/bin:$RUSTUP_HOME/bin:$PATH

# Set up a non-root user
ARG USERNAME=rustuser
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME && \
    apt-get update && apt-get install -y sudo && \
    echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    gcc-arm-none-eabi \
    gdb-multiarch \
    libnewlib-arm-none-eabi \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust and targets
USER $USERNAME
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable && \
    rustup component add rustfmt clippy llvm-tools-preview && \
    rustup target add thumbv7em-none-eabihf thumbv7m-none-eabi

# Install useful cargo utilities
RUN cargo install cargo-binutils cargo-generate probe-rs-cli

# Set working directory
WORKDIR /workspace
```

### 3. **devcontainer.json**
The `devcontainer.json` file configures VS Code settings and installs necessary extensions for Rust and debugging support.

```json
{
    "name": "Embedded Rust DevContainer",
    "build": {
        "dockerfile": "Dockerfile"
    },
    "settings": {
        "terminal.integrated.defaultProfile.linux": "bash",
        "rust-analyzer.cargo.target": "thumbv7em-none-eabihf",
        "lldb.executable": "/usr/bin/gdb-multiarch"
    },
    "extensions": [
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb",
        "ms-vscode.cmake-tools",
        "mutantdino.resourcemonitor"
    ],
    "postCreateCommand": "rustup update",
    "remoteUser": "rustuser",
    "forwardPorts": [],
    "mounts": [
        "source=/dev,target=/dev,type=bind"  // for hardware access if needed
    ]
}
```

### 4. **Key Parts of the Setup**
- **User Permissions**: Runs as a non-root user to avoid permission issues.
- **Target Architecture**: Installs `thumbv7em-none-eabihf` and `thumbv7m-none-eabi` for ARM Cortex-M devices.
- **Tools**: Installs `cargo-binutils` and `probe-rs-cli` to assist with binary inspection and embedded device flashing.
- **Debugger**: `gdb-multiarch` is configured to work with ARM targets.

### 5. **Usage**
1. **Open the project in VS Code**.
2. **Select "Reopen in Container"** to start the development environment.
3. Use the **Rust Analyzer** extension to assist with code completion and project navigation.
4. Connect your embedded device and start debugging with `probe-rs-cli` or `gdb-multiarch`.

Let me know if you have specific configurations or targets you’d like to add!