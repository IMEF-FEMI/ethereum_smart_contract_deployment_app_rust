use crate::*;
use std::{str::FromStr,};

use web3::{
    contract::{Contract, Options},
    types::{H160},
};

pub async fn interact(contract_address: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut selected;

    let http = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(http);
    let accounts = web3.eth().accounts().await?;

    let contract = Contract::from_json(
        web3.eth(),
        H160::from_str(contract_address.as_str()).unwrap(),
        include_bytes!("../../contract/Incrementer.abi"),
    )?;

    loop {
        selected = prompt("select action: \nincrement \nreset \n>");

        if selected.is_empty() {
            continue;
        }
        match selected.as_str() {
            "increment" => {
                selected = prompt("amount: ");
                let amount = selected.parse::<u64>().unwrap();
                // increment
                let tx = contract
                    .call("increment", amount, accounts[0], Options::default())
                    .await?;
                println!("TxHash: {}", tx);
                continue;
            }
            "reset" => {
                // reset
                let tx = contract
                    .call("reset", (), accounts[0], Options::default())
                    .await?;
                println!("TxHash: {}", tx);
                continue;
            }
            _ => println!("Invalid"),
        }
    }
}
