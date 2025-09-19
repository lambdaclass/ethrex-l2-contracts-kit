use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{Parser, arg, command};
use ethrex_l2_rpc::signer::{LocalSigner, Signer};
use ethrex_rpc::EthClient;
use ethrex_sdk::{create2_deploy_from_path, git_clone};
use eyre::Result;
use secp256k1::SecretKey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "http://localhost:1729")]
    rpc_url: String,
    #[arg(long)]
    private_key: String,
    #[arg(long)]
    l1_token: String,
    #[arg(long)]
    salt: String,
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
    let l1_token_address = hex::decode(&args.l1_token).expect("Failed to decode L1 token address");
    constructor_args.extend_from_slice(&l1_token_address);

    let deployer = Signer::Local(LocalSigner::new(SecretKey::from_str(&args.private_key)?));
    let eth_client = EthClient::new("http://localhost:1729")?;
    let contract_path = PathBuf::from("solc_out/WETH9.bin");
    let salt = hex::decode(&args.salt).expect("Failed to decode salt");
    let (tx_hash, deployed_address) = create2_deploy_from_path(
        &constructor_args,
        &contract_path,
        &deployer,
        &salt,
        &eth_client,
    )
    .await?;
    println!("deployed with tx hash {tx_hash:?}");
    println!("deployed at address {deployed_address:?}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    compile_and_deploy(args).await?;
    Ok(())
}
