# 🚀 solana-airdrop

This is a simple Rust based project that demonstrates how to request an airdrop of SOL on the **local Solana test validator (localnet)** using the `@solana/web3.js` library.

It’s meant to be a hands-on, intuitive way to understand how Solana wallets and airdrops work.

---

## 🛠️ What This Project Does

We use TypeScript to:

- Connect to a **local Solana validator** (`localhost:8899`)
- Request an airdrop of SOL to a given public key
- Confirm the transaction

This mimics real-world wallet interactions but on a safe, local blockchain environment.

![Screen Shot 2025-06-04 at 7 54 39 PM](https://github.com/user-attachments/assets/5d7f473a-0184-47bb-882c-46e3369cea35)

---

## 📦 Prerequisites

- Node.js (>= 16)
- `solana-cli` installed
- Local Solana validator running:

```bash
solana-test-validator
