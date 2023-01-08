use std::{str::FromStr, thread, time::Duration};

use web3::{
    contract::{Contract, Options},
    types::{H160, U256},
};

pub async fn monitor_contract(contract_address: String) -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let http = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(http);

    let contract = Contract::from_json(
        web3.eth(),
        H160::from_str(contract_address.as_str()).unwrap(),
        include_bytes!("../../contract/Incrementer.abi"),
    )?;

    let mut previous_value = U256::zero();
    loop {
        let result = contract.query("number", (), None, Options::default(), None);
        let new_value: U256 = result.await?;

        if previous_value != new_value {
            previous_value = new_value;
            println!("new value: {:?}", new_value);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
