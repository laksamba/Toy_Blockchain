use crate::block::Block;
use rusty_leveldb::{DB, Options};
use serde_json; // Ensure this is used for slice conversion

pub struct Blockchain {
    pub db: DB,
    pub last_hash: String,
    pub difficulty: usize,
}

// Iterator to traverse the DB from newest block to Genesis
pub struct BlockchainIterator<'a> {
    current_hash: String,
    db: &'a mut DB,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut opt = Options::default();
        opt.create_if_missing = true;
        // Use a persistent directory name
        let mut db = DB::open("blockchain_db", opt).unwrap();

        // Check if a chain tip already exists under the key "l"
        let last_hash = match db.get(b"l") {
            Some(hash_bytes) => String::from_utf8(hash_bytes).unwrap(),
            None => {
                println!("No existing blockchain found. Generating Genesis...");
                let genesis = Block::new(0, "Genesis Block".into(), "0".into());
                let genesis_hash = genesis.hash.clone();
                let serialized = serde_json::to_vec(&genesis).unwrap();

                // Store Genesis block and the tip
                db.put(genesis_hash.as_bytes(), &serialized).unwrap();
                db.put(b"l", genesis_hash.as_bytes()).unwrap();
                db.flush().unwrap();
                genesis_hash
            }
        };

        Blockchain { db, last_hash, difficulty }
    }

    pub fn add_block(&mut self, data: String) {
        // 1. Fetch the tip of the chain to get the previous block's data
        let last_block_bytes = self.db.get(self.last_hash.as_bytes()).expect("Last block hash not found in DB");
        let last_block: Block = serde_json::from_slice(&last_block_bytes).unwrap();

        // 2. Create and mine the new block
        let mut new_block = Block::new(last_block.index + 1, data, self.last_hash.clone());
        println!("Mining block {}...", new_block.index);
        new_block.mine(self.difficulty);
        println!("Block Mined! Hash: {}", new_block.hash);

        // 3. Update the Database
        let serialized = serde_json::to_vec(&new_block).unwrap();
        self.db.put(new_block.hash.as_bytes(), &serialized).unwrap();
        self.db.put(b"l", new_block.hash.as_bytes()).unwrap();
        self.db.flush().unwrap();

        // 4. Update the local struct tip
        self.last_hash = new_block.hash;
    }

    pub fn validate(&mut self) -> bool {
        let mut iterator = self.iter();

        let mut expected_prev_hash: Option<String> = None;

        while let Some(current_block) = iterator.next() {
            // 1. Verify current block's hash integrity
            if current_block.hash != current_block.calculate_hash() {
                println!("Data corruption detected in block {}", current_block.index);
                return false;
            }

            // 2. Verify link to previous block (if not Genesis)
            if let Some(prev_hash_to_verify) = expected_prev_hash{
                if current_block.hash != prev_hash_to_verify{
                    println!("hash mismatch at block {}",current_block.index + 1);
                return false;
                }
            }

        expected_prev_hash = Some(current_block.previous_hash.clone());
        }
        true
    }

    // Helper to create the iterator
    pub fn iter(&mut self) -> BlockchainIterator<'_> {
        BlockchainIterator {
            current_hash: self.last_hash.clone(),
            db: &mut self.db,
        }
    }
}

// Standard Iterator implementation
impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_hash == "0" {
            None
        } else {
            let block_bytes = self.db.get(self.current_hash.as_bytes())?;
            let block: Block = serde_json::from_slice(&block_bytes).ok()?;
            self.current_hash = block.previous_hash.clone();
            Some(block)
        }
    }
}