mod blockchain;
extern crate serde_derive;
use std::io;
use std::io::Read;
use std::process;
use std::io::Write;

use crate::blockchain::Chain;

fn print_menu() {
    println!("Menu: ");
    println!("1. new transaction");
    println!("2. mine block");
    println!("3. change difficulty");
    println!("4. change reward");
    println!("5. exit");
}

fn read_line_with_message(message: &str, buffer: &mut String) {
    println!("{}", message);
    io::stdout().flush();
    io::stdin().read_line(buffer);
}

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    read_line_with_message("enter miner address", &mut miner_address);
    read_line_with_message("add difficulty", &mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("enter an integer");

    println!("generating genesis block");
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), diff);
    
    loop {
        print_menu();
        choice.clear();
        read_line_with_message("Enter your choice: ", &mut choice);

        match choice.trim().parse::<i32>().unwrap() {
            0 | 5 => {
                println!("Exit!");
                process::exit(0);
            },
            1 => {
                let mut receiver = String::new();
                let mut sender = String::new();
                let mut amount = String::new();

                read_line_with_message("Ender receiver address: ", &mut receiver);
                read_line_with_message("Ender sender address: ", &mut sender);
                read_line_with_message("Ender amount: ", &mut amount);

                let result = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap());

                match result {
                    true => println!("Transaction success"),
                    false => println!("Transaction fail"),
                }

            },
            2 => {
                println!("Generating new block...");
                let result = chain.generate_new_block();

                match result {
                    true => println!("Block generation success"),
                    false => println!("Block generation fail"),
                }
            },
            3 => {
                let mut difficulty = String::new();
                read_line_with_message("Enter difficulty: ", &mut difficulty);

                let result  = chain.update_difficulty(difficulty.trim().parse().unwrap());
                match result {
                    true => println!("Updated difficulty success"),
                    false => println!("Updated difficulty fail"),
                }
            },
            4 => {
                let mut new_reward = String::new();
                read_line_with_message("Enter the new reward", &mut new_reward);

                let result = chain.update_reward(new_reward.trim().parse().unwrap());

                match result {
                    true => println!("Reward update success"),
                    false => println!("Reward update fail"),
                }
            },
            _ => {
                println!("invalid option: {}", choice);
            }
        }
    }
}
