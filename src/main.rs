use std::{env, io::Write, process};

use cli::*;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        println!("🚫 no command entered 🚫");
        process::exit(1);
    } else {
        take_action(&args).await;
    }
    Ok(())
}

async fn take_action(args: &[String]) {
    match args[1].as_str() {
        "deploy" => {
            deploy().await.unwrap();
        }
        "monitor" => {
            let mut contract_address;

            loop {
                contract_address = prompt("contract address:");

                if contract_address.is_empty() {
                    continue;
                }
                break;
            }
            println!("Monitoring {}", contract_address,);

            monitor_contract(contract_address).await.unwrap();
        }
        "interact" => {
            let mut contract_address;

            loop {
                contract_address = prompt("contract address:");

                if contract_address.is_empty() {
                    continue;
                }
                break;
            }
            interact(contract_address).await.unwrap();
        }
        "monitor_async" => {
            let mut contract_address;

            loop {
                contract_address = prompt("contract address:");

                if contract_address.is_empty() {
                    continue;
                }
                break;
            }
            monitor_async(contract_address).await.unwrap();
        }
        _ => println!("Invalid Argument"),
    }
}
fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}
