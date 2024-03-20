use ckb_testtool::ckb_types::prelude::*;
use ckb_testtool::ckb_types::{bytes::Bytes, H256};
use rgbpp_core::schemas::{
    blockchain::*,
    rgbpp::{BTCTimeLockConfig, RGBPPConfig},
};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Output path
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    GenBTCTimeLockConfig {
        #[arg(long)]
        btc_lc_type_hash: String,
    },
    GenRGBPPConfig {
        #[arg(long)]
        version: u16,
        #[arg(long)]
        btc_lc_type_hash: String,
        #[arg(long)]
        btc_time_lock_type_hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let output_path = cli.output.expect("must set output path");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::GenRGBPPConfig {
            version,
            btc_lc_type_hash,
            btc_time_lock_type_hash,
        }) => {
            let config = RGBPPConfig::new_builder()
                .version(version.pack())
                .btc_time_lock_type_hash(string_to_byte32(btc_time_lock_type_hash.as_str()))
                .btc_lc_type_hash(string_to_byte32(btc_lc_type_hash.as_str()))
                .build();
            output_bin_to_file(&output_path, config.as_bytes());
        }
        Some(Commands::GenBTCTimeLockConfig { btc_lc_type_hash }) => {
            let config = BTCTimeLockConfig::new_builder()
                .btc_lc_type_hash(string_to_byte32(btc_lc_type_hash.as_str()))
                .build();
            output_bin_to_file(&output_path, config.as_bytes());
        }
        None => {}
    }
}

fn string_to_byte32(s: &str) -> Byte32 {
    H256::from_str(s.trim_start_matches("0x"))
        .expect("invalid byte32")
        .pack()
}

fn output_bin_to_file<P: AsRef<Path>>(path: P, data: Bytes) {
    let len = data.len();
    std::fs::write(&path, data).expect("write bin to file");

    let path = path.as_ref().as_os_str().to_str().expect("path");
    println!("Written {len} bytes to {path}");
}
