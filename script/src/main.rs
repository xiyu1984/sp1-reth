use clap::Parser;
use sp1_prover::{SP1Prover, SP1Stdin};
use sp1_reth_primitives::SP1RethInput;
use sp1_reth_utils::init::SP1RethInputInitializer;
use sp1_sdk::utils;
use std::fs::File;

/// The version message for the SP1 Reth program.
const VERSION_MESSAGE: &str = concat!(
    "SP1 Reth",
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

/// The ELF file for the SP1 Reth program.
const SP1_RETH_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

/// The CLI arguments for the SP1 Reth program.
#[derive(Parser, Debug)]
#[command(version = VERSION_MESSAGE, about, long_about = None)]
pub struct SP1RethArgs {
    #[arg(short, long)]
    rpc_url: String,

    #[arg(short, long)]
    block_number: u64,

    #[arg(short, long)]
    use_cache: bool,
}

#[tokio::main]
async fn main() {
    // Setup logger.
    sp1_sdk::utils::setup_logger();

    // Parse arguments.
    let args = SP1RethArgs::parse();

    // Get input.
    let input: SP1RethInput = if !args.use_cache {
        let input = SP1RethInput::initialize(&args.rpc_url, args.block_number)
            .await
            .unwrap();
        let mut file =
            File::create(format!("{}.bin", args.block_number)).expect("unable to open file");
        bincode::serialize_into(&mut file, &input).expect("unable to serialize input");
        input
    } else {
        let file = File::open(format!("{}.bin", args.block_number)).expect("unable to open file");
        bincode::deserialize_from(file).expect("unable to deserialize input")
    };

    // Generate proof.
    utils::setup_logger();
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);

    // let prover = SP1Prover::new();
    // let (pk, vk) = prover.setup(SP1_RETH_ELF);

    // let mut core_proof = prover.prove(&pk, stdin).run().unwrap();

    // core_proof.verify(&vk).expect("verification failed");

    // // Save proof.
    // core_proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");
}
