# 🪙 BVP Token – Solana Anchor Staking Program

This repo contains the on-chain program and client tests for **BVP Token**, a staking-enabled token program built using **Rust + Anchor** on the **Solana blockchain**.

> 🚀 Built by Metavibez | CTO of BVP

---

## 🧠 Features

- 🏗 Modular **Anchor program** architecture
- 🪙 **SPL token minting** and staking logic
- 🎯 Tier-based reward system via `tiers` module
- 🔐 Secure initializer with upgrade authority
- ⚙️ **Dockerized** dev/test environment for full isolation
- ✅ Mocha + TypeScript test suite with IDL-bound client
- 🧪 Environment-linked provider/wallet config for easy testing

---

## 📁 Directory Structure

. ├── programs/ │ └── bvp_token/ │ ├── src/ │ │ ├── lib.rs # Main entrypoint │ │ ├── token.rs # Staking + token logic │ │ ├── staking.rs # Stake logic │ │ ├── governance.rs # Voting/governance module │ │ ├── multisig.rs # Multisig logic │ │ └── tiers.rs # Tiered staking reward logic ✅ │ └── Cargo.toml ├── target/types/ # Auto-generated TS types from IDL ├── tests/ # Mocha test suite (initialize, stake) ├── Dockerfile ├── docker-compose.yml ├── Anchor.toml ├── idl.json # IDL w/ program metadata └── package.json

yaml
Copy
Edit


---

## 🧩 Tiers Module

The `tiers.rs` module allows for **dynamic staking tiers** that:

- Assign users to tiers based on stake amount or duration
- Drive reward multipliers
- Can be customized for gamification or governance weight

You can extend `tiers.rs` to add:

- Tier decay
- Dynamic thresholds
- Permissioned tiers

---

## 🔧 Setup Instructions

### 1. Install Dependencies

```bash
npm install
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
ANCHOR_WALLET=/root/.config/solana/id.json
