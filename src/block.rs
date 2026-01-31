use sha2::{Sha256,Digest};
use chrono::Utc;
use serde::{Deserialize,Serialize};

use crate::transaction::Transaction;
use std::fmt;


#[derive(Debug,Clone,Serialize,Deserialize)]


pub struct Block{
    pub index:u32,
    pub timestamp:i64,
   pub transactions:Vec<Transaction>,
    pub previous_hash:String,
    pub hash:String,
    pub nonce:u64,
}

impl Block {
    pub fn new(index:u32,transactions:Vec<Transaction>,previous_hash:String) -> Self{
        let mut block = Block{
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash:String::new(),
            nonce:0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self)-> String{
        let tx_json = serde_json::to_string(&self.transactions)
        .expect("failed to serialized transaction");
        let input = format!("{}{}{}{}{}",self.index,self.timestamp,tx_json,self.nonce,self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:X}",hasher.finalize())
    }


    pub fn mine(&mut self, difficulty:usize){
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target{
            self.nonce += 1;
            self.hash = self.calculate_hash();
             if self.nonce % 1_000_000 == 0 {
        println!("Tried {} million nonces...", self.nonce / 1_000_000);
    }
        }
    }


}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Block {}]", self.index)?;
        writeln!(f, "Hash: {}", self.hash)?;
        writeln!(f, "Previous Hash: {}", self.previous_hash)?;
        writeln!(f, "Nonce: {}", self.nonce)?;

        if self.transactions.is_empty() {
            writeln!(f, "Transactions: []")
        } else {
            writeln!(f, "Transactions:")?;
            for (i, tx) in self.transactions.iter().enumerate() {
                writeln!(f, "  TX #{}:\n    {}", i + 1, tx)?;
            }
            Ok(())
        }
    }
}