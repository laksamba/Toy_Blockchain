use sha2::{Sha256,Digest};
use chrono::Utc;


#[derive(Debug,Clone)]

pub struct Block{
    pub index:u32,
    pub timestamp:i64,
    pub data:String,
    pub previous_hash:String,
    pub hash:String,
    pub nonce:u64,
}

impl Block {
    pub fn new(index:u32,data:String,previous_hash:String) -> Self{
        let mut block = Block{
            index,
            timestamp: Utc::now().timestamp(),
            data,
            previous_hash,
            hash:String::new(),
            nonce:0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self)-> String{
        let input = format!("{}{}{}{}{}",self.index,self.timestamp,self.data,self.nonce,self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:X}",hasher.finalize())
    }


    pub fn mine(&mut self, difficulty:usize){
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target{
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }


}