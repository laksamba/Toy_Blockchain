use crate::block::Block;


pub struct Blockchain{
    pub chain:Vec<Block>,
    pub difficulty:usize,
}


impl Blockchain {
    pub fn new(difficulty:usize)->Self{
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain { chain: vec![genesis], difficulty }
    }

    pub fn add_block(&mut self,data:String){
        let last_block = self.chain.last().unwrap();
        let mut new_block = Block::new(self.chain.len() as u32, data, last_block.hash.clone());
        
        println!("mining block {}....",new_block.index);
        new_block.mine(self.difficulty);
        println!("Block Mined Hash:{}",new_block.hash);

        self.chain.push(new_block);

    }

    pub fn validate(&self)->bool{
        for i in 1..self.chain.len(){
            if self.chain[i].hash != self.chain[i].calculate_hash() {
                return false;
            }
            if self.chain[i].previous_hash != self.chain[i-1].hash{
                return false;
            }
        }
        true
    }
}