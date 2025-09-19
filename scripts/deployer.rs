use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use ethrex_l2_rpc::signer::{LocalSigner, Signer};
use ethrex_rpc::EthClient;
use ethrex_sdk::{create2_deploy_from_path, git_clone};
use eyre::Result;
use secp256k1::SecretKey;

async fn compile_contracts() -> Result<()> {
    // Logic to compile contracts
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
    let l1_token_address = hex::decode("178a38b8f3f1b1fa632c95c60519bb77bd041074")
        .expect("Failed to decode L1 token address");
    constructor_args.extend_from_slice(&l1_token_address);

    let deployer = Signer::Local(LocalSigner::new(SecretKey::from_str(
        "bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31",
    )?));
    let eth_client = EthClient::new("http://localhost:1729")?;
    let contract_path = PathBuf::from("solc_out/WETH9.bin");
    let salt = [0u8; 32];
    let a = create2_deploy_from_path(
        &constructor_args,
        &contract_path,
        &deployer,
        &salt,
        &eth_client,
    )
    .await?;
    dbg!(a.1);
    dbg!(a.0);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    compile_contracts().await?;
    Ok(())
}
