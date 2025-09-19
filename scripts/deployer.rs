use std::{
    fs,
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
    let output_dir = Path::new("out");
    let contract_path = Path::new("lib/contracts/token/ERC20/ERC20.sol");
    compile_contract_to_bytecode(
        output_dir,
        contract_path,
        "ERC20",
        false,
        None,
        &[output_dir],
    );

    let remappings = [("@openzeppelin/contracts", PathBuf::from("lib/contracts"))];
    let contract_path = Path::new("../contracts/WETH9.sol");
    let contract_name = "WETH9";
    compile_contract_to_bytecode(
        output_dir,
        contract_path,
        contract_name,
        false,
        Some(&remappings),
        &[
            output_dir,
            Path::new("contracts"),
            Path::new("lib/contracts"),
        ],
    );

    // println!("Compiling {contract_name} contract");
    // ethrex_sdk::compile_contract(
    //     output_dir,
    //     contract_path,
    //     false,
    //     Some(&remappings),
    //     &[
    //         output_dir,
    //         Path::new("contracts"),
    //         Path::new("lib/contracts"),
    //     ],
    // )
    // .expect("Failed to compile contract");
    // println!("Successfully compiled {contract_name} contract");

    // // Append L1 token to the bytecode
    // let mut bytecode = fs::read(output_dir.join("solc_out").join("WETH9.bin"))
    //     .expect("Failed to read bytecode file");
    let mut constructor_args = vec![0u8; 12];
    let l1_token_address = hex::decode("178a38b8f3f1b1fa632c95c60519bb77bd041074")
        .expect("Failed to decode L1 token address");
    constructor_args.extend_from_slice(&l1_token_address);
    // bytecode.extend_from_slice(&padding);
    // bytecode.extend_from_slice(&l1_token_address);

    let deployer = Signer::Local(LocalSigner::new(SecretKey::from_str(
        "bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31",
    )?));
    let eth_client = EthClient::new("http://localhost:1729")?;
    let contract_path = PathBuf::from("out/solc_out/WETH9.bin");
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
    // let _ = deploy_with_proxy_from_bytecode(&deployer, &eth_client, &bytecode, &salt).await;
    Ok(())
}

fn compile_contract_to_bytecode(
    output_dir: &Path,
    contract_path: &Path,
    contract_name: &str,
    runtime_bin: bool,
    remappings: Option<&[(&str, PathBuf)]>,
    allow_paths: &[&Path],
) {
    println!("Compiling {contract_name} contract");
    ethrex_sdk::compile_contract(
        output_dir,
        contract_path,
        runtime_bin,
        remappings,
        allow_paths,
    )
    .expect("Failed to compile contract");
    println!("Successfully compiled {contract_name} contract");

    // Resolve the resulted file path
    let filename = if runtime_bin {
        format!("{contract_name}.bin-runtime")
    } else {
        format!("{contract_name}.bin")
    };
    let file_path = output_dir.join("solc_out").join(&filename);

    // Get the output file path
    let output_file_path = output_dir
        .join("solc_out")
        .join(format!("{contract_name}.bytecode"));

    decode_to_bytecode(&file_path, &output_file_path);

    println!("Successfully generated {contract_name} bytecode");
}

fn decode_to_bytecode(input_file_path: &Path, output_file_path: &Path) {
    let bytecode_hex = fs::read_to_string(input_file_path).expect("Failed to read file");

    let bytecode = hex::decode(bytecode_hex.trim()).expect("Failed to decode bytecode");

    fs::write(output_file_path, bytecode).expect("Failed to write bytecode");
}

#[tokio::main]
async fn main() -> Result<()> {
    compile_contracts().await?;
    Ok(())
}
