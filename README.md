# 🦀 Rust-Chain: A Modular Toy Blockchain

A lightweight, single-node blockchain simulation written in Rust. This project demonstrates the core mechanics of a decentralized ledger, including **Proof of Work (PoW)**, **Block Hashing**, and **Chain Validation**, all through an interactive CLI.

---

## 🏗️ Architecture

The project is split into three distinct modules to simulate a real-world node structure:

- **`Block` Module**: Handles the data structure, SHA-256 hashing logic, and the mining loop.
- **`Blockchain` Module**: Manages the ledger state, enforces the consensus rules (difficulty), and handles block validation.
- **`Main/CLI` Module**: An interactive dashboard for the node operator to interact with the chain in real-time.



---

## ✨ Features

- **Interactive CLI**: Powered by `inquire`, offering a menu-driven experience.
- **Customizable Difficulty**: Adjust the mining difficulty to see how it impacts CPU work.
- **Immutable Ledger**: Any tampering with block data invalidates the entire chain.
- **Proof of Work**: Implements a simple "leading zeros" hashing challenge.

---

## 🚀 Getting Started

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation
1. Clone the repository:
   ```bash
   git clone [https://github.com/laksamba/Toy_Blockchain.git]
   cd Toy_Blockchain

2. Build the project:
   ```bash
   cargo build --release


   Running the Node :
   ```bash
   cargo run
   
3. Usage:
   Once the node starts, you can:

   Add Transaction & Mine: Enter data (e.g., "Alice -> Bob: 10 BTC") and watch the miner solve the hash.

   View Chain: Inspect the current state of the ledger, including hashes, nonces, and timestamps.

   Validate Chain: Run a diagnostic to ensure the hash pointers are intact and no data has been corrupted.