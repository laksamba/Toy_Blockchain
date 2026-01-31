mod block;
mod blockchain;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use inquire::{Select, Text};

use crate::transaction::Transaction;

fn main() {
    let my_wallet = wallet::Wallet::new();
    println!("Wallet generated. Your Address:{}", my_wallet.get_address());

    // crate a temporary transaction to get the data bytes

    let mut node = Blockchain::new(5);

    println!("Rust node Initialized (Porwered by levelDb)");

    loop {
        let options = vec![
            "Add Transaction & mine",
            "view Chain",
            "validateChain",
            "Exit",
        ];
        let ans = Select::new("Node Operation:", options).prompt();

        match ans {
            Ok("Add Transaction & mine") => {
                let receiver = Text::new("Receiver Address:").prompt().unwrap();
                let amount: u64 = loop {
                    let input = Text::new("Amount (number only):").prompt().unwrap();

                    match input.trim().parse() {
                        Ok(v) => break v,
                        Err(_) => println!("❌ Please enter a valid number (e.g. 5, 10, 100)"),
                    }
                };

                // temporary tansaction to get the data bytes
                let sender_addr = my_wallet.get_address();
                let temp_tx =
                    Transaction::new(sender_addr.clone(), receiver.clone(), amount, "".into());
                //    sign transaction
                let signature = my_wallet.sign(&temp_tx.to_bytes());
                //    create final transaction
                let final_tx = Transaction::new(sender_addr, receiver, amount, signature);

                node.add_block(vec![final_tx]);
            }

            Ok("view Chain") => {
                for block in node.get_all_blocks() {
                    println!("{}", block);
                }

                println!("\n ----end of Chain----");
            }
            Ok("validateChain") => {
                if node.validate() {
                    println!("🟢 Chain is secure. All hashes verified against disk.")
                } else {
                    println!("🔴 ALERT: Chain integrity compromised!")
                }
            }
            Ok("Exit") | Err(_) => {
                println!("shutting down Node ....");
                break;
            }
            _ => unreachable!(),
        }
    }
}
