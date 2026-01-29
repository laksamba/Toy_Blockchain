mod block;
mod blockchain;


use blockchain::Blockchain;
use inquire::{Select,Text};

fn main(){
    let mut node = Blockchain::new(5);

    println!("Rust node Initialized (Porwered by levelDb)");

    loop {
        let options = vec!["Add Transaction & mine", "view Chain", "validateChain","Exit"];
        let ans = Select::new("Node Operation:",  options).prompt();


        match ans {
            Ok("Add Transaction & mine")=>{
                let data = Text::new("Enter transaction data").prompt().unwrap();
                node.add_block(data);

            }

            Ok("view Chain")=>{
                for block in node.iter(){
                    println!("\n[Block {}]", block.index);
                    println!("Hash: {}", block.hash);
                    println!("Data: {}", block.data);
                    println!("Previous Hash: {}",block.previous_hash);
                    println!("Nonce: {}", block.nonce);
                }
                println!("\n ----end of Chain----");
            }
            Ok("validateChain")=>{
                if node.validate(){
                    println!("🟢 Chain is secure. All hashes verified against disk.")
                }else {
                    println!("🔴 ALERT: Chain integrity compromised!")
                }
            }
            Ok("Exit") | Err(_) => {
                println!("shutting down Node ....");
                break;
            },
            _=> unreachable!(),
        }
    }

}

