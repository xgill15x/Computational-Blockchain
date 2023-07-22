
#[cfg(test)]
mod block_tests {
    use crate::block::*;

    #[test]
    fn hash_string() { // testing hash_string and hash_string_for_proof
        let mut block: Block = Block::initial(16);
        block.set_proof(56231);

        let initial_hash_string: String = format!("0000000000000000000000000000000000000000000000000000000000000000:0:16::56231");
        
        assert_eq!(block.hash_string_for_proof(56231), initial_hash_string);
        assert_eq!(block.hash_string(), initial_hash_string);   // panics if proof value is None
    }

    #[test]
    fn hash_block() {   // testing hash_for_proof and hash 
        let mut block: Block = Block::initial(16);
        block.set_proof(56231);

        let first_valid_hash: String = format!("6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000");
        
        // a single worker should mine the first valid proof 56231 to get first valid hash
        assert_eq!(format!("{:02x}", block.hash_for_proof(56231)), first_valid_hash);   // hash_for_proof returns expected hash value for given proof
        assert_eq!(format!("{:02x}", block.hash()), first_valid_hash);  // hash shouldn't panic as long as the proof is not None
    }

    #[test]
    fn init_block() {
        let block: Block = Block::initial(20);
        assert_eq!(block.prev_hash, Hash::from([0u8; 32]));
        assert_eq!(block.generation, 0);
        assert_eq!(block.difficulty, 20);
        assert_eq!(block.data, format!(""));
        assert_eq!(block.proof, None);
    }

    #[test]
    fn next_block() {
        let mut block: Block = Block::initial(16);
        block.set_proof(56231);
        
        let block2: Block = Block::next(&block, format!("SecretMessage"));
        assert_eq!(block2.prev_hash, block.hash());
        assert_eq!(block2.generation, 1);
        assert_eq!(block2.difficulty, 16);
        assert_eq!(block2.data, format!("SecretMessage"));
        assert_eq!(block2.proof, None);
    }

    #[test]
    fn proof_validity() {  // testing is_valid_for_proof and is_valid
        let mut block: Block = Block::initial(16);

        assert!(block.is_valid_for_proof(56231));   // 56231 and 60515 should both be valid as their hashes end with 16 0 bits
        assert!(block.is_valid_for_proof(60515));
        assert!(!block.is_valid_for_proof(2));  // invalid proof value since its hash doesn't end with 16 0 bits

        assert!(!block.is_valid()); // block shouldn't be valid since we haven't set a valid proof value
        
        block.set_proof(60515);
        assert!(block.is_valid());  // the proof value should result in the block having a valid hash
    }

    #[test]
    fn mine_proof() {   // testing mine and mine_for_proof
        let mut block: Block = Block::initial(16);

        assert_eq!(block.mine_for_proof(1), 56231);   // mine_for_proof should identify 56231 to be the first valid proof for d=16
        
        assert!(!block.is_valid()); // mine_for_proof shouldn't make the block valid
        
        block.mine(1);
        assert!(block.is_valid());  // mine should set the proof value and make the block valid
    }

}
