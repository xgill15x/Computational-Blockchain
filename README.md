# PoW-Blockchain

This is a simple blockchain implementation in Rust that demonstrates the creation, mining, and validation of blocks. The program features a proof-of-work mechanism to secure the blockchain.

## Features

- **Block Creation**: Generate the genesis block and subsequent blocks.
- **Mining**: Solve the proof-of-work problem to find a valid proof.
- **Validation**: Verify the validity of each block in the chain.
- **Concurrency**: Mine blocks using multiple threads for faster computation.

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/Computational-Blockchain.git
   cd Computational-Blockchain/src
2. Install dependencies via Cargo.toml
   ```bash
   cargo build
3. Run the demo program
   ```bash
   cargo run

The program will:
- Create the genesis block.
- Mine the genesis block and additional blocks.
- Validate the blockchain.
- Print block details, including hashes, proofs, and data.
