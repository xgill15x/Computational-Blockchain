use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
// use std::fmt::Write;
use std::sync;

pub type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    pub prev_hash: Hash,
    pub generation: u64,
    pub difficulty: u8,
    pub data: String,
    pub proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        Block {
            prev_hash: Hash::from([0u8; 32]),
            generation: 0,
            difficulty: difficulty,
            data: format!(""),
            proof: None
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        Block {
            prev_hash: previous.hash(),
            generation: previous.generation + 1,
            difficulty: previous.difficulty,
            data: data,
            proof: None
        }
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String { // hash string of the block
        let prev_hash_str: String = format!("{:02x}", self.prev_hash).to_lowercase();
        let generation_str: String = self.generation.to_string();
        let difficulty_str: String = self.difficulty.to_string();
        let proof_str: String = proof.to_string();

        format!("{}:{}:{}:{}:{}", prev_hash_str, generation_str, difficulty_str, self.data, proof_str)
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block hasn't been mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {  // the actual hash for the block
        let mut hasher: Sha256 = Sha256::new();
        let proof_str: String = self.hash_string_for_proof(proof);

        hasher.update(proof_str);
        hasher.finalize()
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block hasn't been mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {  // checks if hash's last `self.difficulty` bits are 0s
        let n_bytes: u8 = self.difficulty/8;
        let n_bits: u8 = self.difficulty%8;
        
        let potential_block_hash = self.hash_for_proof(proof);  // creating hash from the potential proof value

        let difficulty_bytes_start: usize = potential_block_hash.len() - n_bytes as usize;

        //if any of last n bytes are not 0u8, the not valid
        for byte_idx in difficulty_bytes_start..potential_block_hash.len() {
            if potential_block_hash[byte_idx] != 0u8 {
                return false
            }
        }

        if potential_block_hash[difficulty_bytes_start-1] % (1<<n_bits) != 0 {
            return false
        }

        true
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        let mut work_queue: WorkQueue<MiningTask> = WorkQueue::new(workers);

        // amount of work each thread does given `chunks` sections
        let worker_load: u64 = (end - start) / chunks;
        
        let mut current_pos: u64 = start;

        let mut reached_last_chunk: bool = false; // used to bound work in the last chunk

        // - atomic-referencel-counted instance for thread safety
        let shared_block: sync::Arc<Block> = sync::Arc::new(self.clone());

        for _ in 0..chunks {
            let block_clone: sync::Arc<Block> = shared_block.clone();
            
            let mut boundary: u64 = current_pos + worker_load;
            if boundary > end {  // for when the subdivision overflows the given range
                reached_last_chunk = true;
                boundary = end;
            }

            work_queue.enqueue(MiningTask::new(block_clone, current_pos, boundary)).expect("Error enqueing mining task");

            current_pos += worker_load+1;

            if reached_last_chunk {
                break;
            }
        }
        
        let valid_proof: u64 = work_queue.recv();   // blocks until first valid result
        work_queue.shutdown(); // stop threads from doing unecessary work
        
        valid_proof
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    start_range: u64,
    end_range: u64
}

impl MiningTask {
    pub fn new(block: sync::Arc<Block>, start_range: u64, end_range: u64) -> MiningTask {
        MiningTask { block, start_range, end_range }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        for proof_val in self.start_range..=self.end_range {
            if self.block.is_valid_for_proof(proof_val) {
                return Some(proof_val);
            }
        }
        None
    }
}
