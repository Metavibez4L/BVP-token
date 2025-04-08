FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
  curl gnupg2 ca-certificates build-essential pkg-config libssl-dev libudev-dev \
  libclang-dev cmake git unzip wget libpq-dev libssl-dev llvm \
  && apt-get clean

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain 1.74.1
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.17.28/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.29.0 anchor-cli --locked

# Install Node.js 18.x and npm
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
  && apt-get install -y nodejs \
  && npm install -g npm

# Create app directory
WORKDIR /app

# Copy and install TypeScript project dependencies
COPY package.json tsconfig.json ./
RUN npm install

# Copy the rest of the project
COPY . .

# Verify versions
RUN anchor --version && solana --version && rustc --version && cargo --version && node -v && npm -v

CMD ["bash"]