use std::{ str::FromStr, thread, time::Duration};

use web3::{
    contract::{Contract, Options},
    types::{H160, U256},
};



pub async fn monitor_async(contract_address: String) -> Result<(), Box<dyn std::error::Error>> {
    let http = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(http);
    let accounts = web3.eth().accounts().await?;

    let contract = Contract::from_json(
        web3.eth(),
        H160::from_str(contract_address.as_str()).unwrap(),
        include_bytes!("../../contract/Incrementer.abi"),
    )?;



    let contract_clone = contract.clone();
    let handle = tokio::spawn(async move {
       
        contract_clone.call("increment", 4u16, accounts[0], Options::default()).await.unwrap();
        thread::sleep(Duration::from_secs(2));
        contract_clone.call("increment", 4u16, accounts[0], Options::default()).await.unwrap();
        thread::sleep(Duration::from_secs(2));
        contract_clone.call("reset", (), accounts[0], Options::default()).await.unwrap();

    });
    
    let mut previous_value = U256::zero();
    loop {

        thread::sleep(Duration::from_secs(2));

        let result = contract.query("number", (), None, Options::default(), None);
        let new_value: U256 = result.await?;

        if previous_value != new_value {
            previous_value = new_value;
            println!("new value: {:?}", new_value);
        }
        if new_value == U256::zero() {
            println!("User has done a reset. Shutting down program......");
            break;
        }
    }
    handle.await.unwrap();

    Ok(())
}
