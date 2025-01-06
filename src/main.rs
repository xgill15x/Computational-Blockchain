mod queue;
mod block;

use crate::block::Block;

fn main() {
    // Number of workers for mining
    let workers = 4;

    // Initializing the blockchain
    let mut blockchain = Vec::new();
    let difficulty = 4;

    println!("Creating the initial block...");
    let mut genesis_block = Block::initial(difficulty);
    genesis_block.mine(workers); // Mine the genesis block
    println!(
        "Genesis block mined: Hash = {:02x}, Proof = {}",
        genesis_block.hash(),
        genesis_block.proof.unwrap()
    );
    blockchain.push(genesis_block);

    // Add new blocks to the blockchain
    for i in 1..4 {
        println!("\nCreating block {}...", i);
        let previous_block = &blockchain[blockchain.len() - 1];
        let mut new_block = Block::next(previous_block, format!("Block #{}", i));
        new_block.mine(workers); // Mine the new block
        println!(
            "Block {} mined: Hash = {:02x}, Proof = {}",
            i,
            new_block.hash(),
            new_block.proof.unwrap()
        );
        blockchain.push(new_block);
    }

    // Verify the blockchain
    println!("\nVerifying blockchain...");
    for (i, block) in blockchain.iter().enumerate() {
        if block.is_valid() {
            println!("Block {} is valid.", i);
        } else {
            println!("Block {} is invalid!", i);
        }
    }

    println!("\nBlockchain demonstration complete!");
}
