use std::path::{Path, PathBuf};

use clap::{Parser, arg, command};
use ethereum_types::{H160, H256};
use ethrex_l2_rpc::signer::{LocalSigner, Signer};
use ethrex_rpc::EthClient;
use ethrex_sdk::{create2_deploy_from_path, git_clone};
use eyre::Result;
use hex::FromHexError;
use reqwest::Url;
use secp256k1::SecretKey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "http://localhost:1729")]
    rpc_url: Url,
    #[arg(long, value_parser = parse_private_key)]
    private_key: SecretKey,
    #[arg(long)]
    l1_token: H160,
    #[arg(long)]
    salt: H256,
}

async fn compile_and_deploy(args: Args) -> Result<()> {
    git_clone(
        "https://github.com/OpenZeppelin/openzeppelin-contracts.git",
        "lib",
        None,
        true,
    )?;
    let output_dir = Path::new(".");
    let contract_path = Path::new("lib/contracts/token/ERC20/ERC20.sol");
    ethrex_sdk::compile_contract(output_dir, contract_path, false, None, &[output_dir])
        .expect("Failed to compile contract");

    let remappings = [("@openzeppelin/contracts", PathBuf::from("lib/contracts"))];
    let contract_path = Path::new("../contracts/WETH9.sol");
    ethrex_sdk::compile_contract(
        output_dir,
        contract_path,
        false,
        Some(&remappings),
        &[
            output_dir,
            Path::new("contracts"),
            Path::new("lib/contracts"),
        ],
    )
    .expect("Failed to compile contract");
    let mut constructor_args = vec![0u8; 12];
    let l1_token_address = args.l1_token.as_bytes();
    constructor_args.extend_from_slice(l1_token_address);

    let deployer = Signer::Local(LocalSigner::new(args.private_key));
    let eth_client = EthClient::new(args.rpc_url.as_str())?;
    let contract_path = PathBuf::from("solc_out/WETH9.bin");
    let salt = &args.salt.as_bytes();
    let (tx_hash, deployed_address) = create2_deploy_from_path(
        &constructor_args,
        &contract_path,
        &deployer,
        salt,
        &eth_client,
    )
    .await?;
    println!("deployed with tx hash {tx_hash:?}");
    println!("deployed at address {deployed_address:?}");
    Ok(())
}

pub fn parse_hex(s: &str) -> eyre::Result<Vec<u8>, FromHexError> {
    match s.strip_prefix("0x") {
        Some(s) => hex::decode(s),
        None => hex::decode(s),
    }
}

pub fn parse_private_key(s: &str) -> eyre::Result<SecretKey> {
    Ok(SecretKey::from_slice(&parse_hex(s)?)?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    compile_and_deploy(args).await?;
    Ok(())
}
