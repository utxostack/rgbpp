use ckb_testtool::ckb_types::core::ScriptHashType;
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
    CalcScriptHash {
        #[arg(long)]
        code_hash: String,
        #[arg(long)]
        hash_type: String,
        #[arg(long)]
        args: String,
    },
}

fn main() {
    let cli = Cli::parse();

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
            let output_path = cli.output.expect("must set output path");
            output_bin_to_file(output_path, config.as_bytes());
        }
        Some(Commands::GenBTCTimeLockConfig { btc_lc_type_hash }) => {
            let config = BTCTimeLockConfig::new_builder()
                .btc_lc_type_hash(string_to_byte32(btc_lc_type_hash.as_str()))
                .build();
            let output_path = cli.output.expect("must set output path");
            output_bin_to_file(output_path, config.as_bytes());
        }
        Some(Commands::CalcScriptHash {
            code_hash,
            hash_type,
            args,
        }) => {
            let code_hash = string_to_byte32(code_hash);
            let args = string_to_bytes(args);
            let hash_type = match hash_type.to_lowercase().as_str() {
                "data" => ScriptHashType::Data,
                "type" => ScriptHashType::Type,
                "data1" => ScriptHashType::Data1,
                "data2" => ScriptHashType::Data2,
                _ => panic!("unexpected hash_type {}", hash_type),
            };
            let script = Script::new_builder()
                .code_hash(code_hash)
                .args(args.pack())
                .hash_type(hash_type.into())
                .build();
            let script_hash = script.calc_script_hash();
            println!("Script:");
            println!("{script}");
            println!("Output hash:");
            println!("{script_hash:#}");
        }
        None => {}
    }
}

fn string_to_byte32(s: &str) -> Byte32 {
    H256::from_str(s.trim_start_matches("0x"))
        .expect("invalid byte32")
        .pack()
}

fn string_to_bytes(s: &str) -> Bytes {
    hex::decode(s.trim_start_matches("0x"))
        .expect("invalid bytes")
        .into()
}

fn output_bin_to_file<P: AsRef<Path>>(path: P, data: Bytes) {
    let len = data.len();
    std::fs::write(&path, data).expect("write bin to file");

    let path = path.as_ref().as_os_str().to_str().expect("path");
    println!("Written {len} bytes to {path}");
}
