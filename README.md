# 🎬 BVP Token Smart Contract

This repository contains the on-chain Solana program for the **BVP Cryptosystem**, written using the [Anchor framework](https://book.anchor-lang.com/). It powers the decentralized infrastructure for **Big Vision Pictures (BVP)** — enabling token-based access, staking rewards, multi-sig treasury security, and tier-based experience gating for token holders.

---

## 🚀 Features

### 🧾 Token Logic
- SPL-compatible `$BVP` token
- Mint, transfer, and tokenomics integration
- Configurable supply, max wallet, and transaction limits

### 🧱 Staking System
- **Reward Staking**: Earn interest for locking tokens (up to 8% APR)
- **Experience Staking**: Gate access to exclusive BVP film experiences
- Auto-compounding & duration-based reward tiers

### 🎭 Tier System
- Assigns holder benefits based on $BVP token balances
- Supports Bronze, Silver, Gold, Platinum, and Diamond tiers
- Unlock real-world experiences (red carpet, production access, cameos, more)

### 🔐 Multisig Treasury Security
- Multi-signature control of operational and treasury wallets
- Threshold logic for transfers and spending limits
- Cold storage & hot wallet separation strategy

### 📜 Governance Module
- Founders-only governance structure
- Special proposal voting for token holders
- Future extension point for DAO-style logic

---

## 📦 Project Structure
programs/bvp_token/ ├── lib.rs # Main entry and module loader ├── token.rs # Token logic ├── staking.rs # Reward and experience staking logic ├── multisig.rs # Multi-sig fund management ├── governance.rs # Proposal and voting system └── tiers.rs # Tier classification system

tests/ └── bvp_token.ts # Mocha/Chai-based integration test

Anchor.toml # Anchor configuration Cargo.toml # Rust crate config tsconfig.json # TypeScript test setup package.json # Test runner config (Mocha + ts-node)


---

## ⚙️ Development Setup

### 📥 Clone and Install

```bash
git clone https://github.com/Metavibez4L/BVP-token.git
cd BVP-token
npm install

