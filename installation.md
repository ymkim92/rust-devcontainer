Certainly! Here's a `Dockerfile` for setting up a VS Code **devcontainer** with Rust development tools based on **Ubuntu 22.04**.

### Dockerfile for Rust Development

```Dockerfile
# Use Ubuntu 22.04 as the base image
FROM ubuntu:22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install required dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    git \
    ca-certificates \
    --no-install-recommends

# Install Rust (Stable) using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set the environment path for Rust binaries
ENV PATH=/root/.cargo/bin:$PATH

# Install some useful Rust components and tools
RUN rustup component add rustfmt clippy

# Install VSCode Rust extensions dependencies (optional but useful)
RUN apt-get install -y \
    lldb \
    python3 \
    python3-pip \
    --no-install-recommends

# Clean up APT cache
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

# Verify installation by showing rustc and cargo version
RUN rustc --version && cargo --version

# Set working directory
WORKDIR /workspace

# Expose port 3000 (example if you are running a web app)
EXPOSE 3000
```

### Setting up the DevContainer Configuration

For VS Code to use this `Dockerfile`, you also need a `.devcontainer/devcontainer.json` file. Here's how it can look:

```json
{
    "name": "Rust DevContainer",
    "build": {
        "dockerfile": "Dockerfile"
    },
    "settings": {
        "terminal.integrated.defaultProfile.linux": "/bin/bash"
    },
    "extensions": [
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb",
        "mutantdino.resourcemonitor"
    ],
    "forwardPorts": [3000],
    "postCreateCommand": "rustup update"
}
```

### Explanation:

1. **Base Image**: Starts with Ubuntu 22.04 as the base.
2. **Dependencies**: Installs essential development tools (`build-essential`, `curl`, etc.), Rust (via `rustup`), and optional debugging tools like `lldb`.
3. **Rust Installation**: Installs Rust (including `cargo`, `rustfmt`, and `clippy`) using `rustup`.
4. **PATH**: Adds Rust binaries to the system path.
5. **Workspace**: The working directory is set to `/workspace`, which is where your code will reside.
6. **Extensions**: The `devcontainer.json` installs useful VS Code extensions for Rust development like `rust-analyzer` and `lldb` for debugging.
7. **Port Forwarding**: The example exposes port 3000, which you can change based on your application’s needs.

### Usage:

1. Create a `.devcontainer` folder in your project.
2. Place the `Dockerfile` and `devcontainer.json` files inside it.
3. Open the project in VS Code and choose "Reopen in Container" when prompted.

This will set up a development environment using Docker, tailored for Rust development with VS Code.