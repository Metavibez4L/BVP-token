FROM archlinux:latest

ENV PATH="/root/.cargo/bin:/root/.avm/bin:/root/.local/share/solana/install/active_release/bin:$PATH"

# Install system dependencies
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm git curl base-devel openssl llvm clang cmake nodejs npm python

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y && rustup default stable

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.16.18/install)"

# Install Anchor using AVM
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked && \
    avm install 0.31.0 && \
    avm use 0.31.0

# Node toolchain (for Anchor test/dev env)
RUN npm install -g typescript yarn ts-node mocha ts-mocha

# Create a project folder
WORKDIR /project
CMD ["/bin/bash"]
