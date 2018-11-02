extern crate web3;

use std::str::FromStr;
use web3::futures::Future;
use web3::types::{H160, U256};
use web3::Error;

fn get_balance(
    web3: &web3::Web3<web3::transports::WebSocket>,
    address: &web3::types::Address,
) -> Result<U256, Error> {
    web3.eth().balance(*address, None).wait()
}

pub fn run() {
    let network = if cfg!(debug_assertions) {
        "ws://localhost:7545"
    } else {
        "wss://mainnet.infura.io/ws"
    };

    let (_eloop, transport) = match web3::transports::WebSocket::new(network) {
        Ok((l, t)) => (l, t),
        Err(err) => {
            println!("Cannot connect due {:?}", err);
            std::process::exit(-1)
        }
    };

    let web3 = web3::Web3::new(transport);

    let account_str = match std::env::args().skip(1).last() {
        Some(v) => v,
        None => {
            println!("Usage: {} <address>", std::env::args().next().unwrap());
            std::process::exit(-1)
        }
    };

    let account = match H160::from_str(&account_str[2..]) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid account");
            std::process::exit(-1)
        }
    };

    let balance = match get_balance(&web3, &account) {
        Ok(value) => value,
        Err(err) => {
            println!("Balance not found for {:?} cause {:?}", account, err);
            std::process::exit(-1)
        }
    };

    println!("{}", balance);
}
