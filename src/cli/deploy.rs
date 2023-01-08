use core::time;

use web3::{
    contract::{Contract, Options},
};

pub async fn deploy() -> Result<(), Box<dyn std::error::Error>> {

    let http = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(http);
    let accounts = web3.eth().accounts().await?;
    // Get current balance
    let balance = web3.eth().balance(accounts[0], None).await?;

    println!("Balance: {}", balance); // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("../../contract/Incrementer.bin");

    // Deploying a contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("../../contract/Incrementer.abi"))?
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, 0u64, accounts[0])
        .await?;

    println!("Deployed at: {}", contract.address().to_string());




    Ok(())
}
// solc -o contract  --bin --abi contract/*.sol --overwrite
