mod block;
mod blockchain;


use blockchain::Blockchain;
use inquire::{Select,Text};

fn main(){
    let mut node = Blockchain::new(4);
    println!("Rust note initialized");

    loop {
        let options = vec!["Add Transaction & mine", "view Chain", "validateChain","Exit"];
        let ans = Select::new("Node Operation:",  options).prompt();


        match ans {
            Ok("Add Transaction & mine")=>{
                let data = Text::new("Enter transaction data").prompt().unwrap();
                node.add_block(data);
            }

            Ok("view Chain")=>{
                for block in &node.chain{
                    println!("\n[Block {}]", block.index);
                    println!("Hash: {}", block.hash);
                    println!("Data: {}", block.data);
                    println!("Nonce: {}", block.nonce);
                }
            }
            Ok("validateChain")=>{
                if node.validate(){
                    println!("🟢 Chain is secure.")
                }else {
                    println!("🔴 ALERT: Chain integrity compromised!")
                }
            }
            Ok("Exit") | Err(_) => break,
            _=> unreachable!(),
        }
    }

}

