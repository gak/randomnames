#![feature(error_generic_member_access)]

use std::backtrace::Backtrace;

use rand::{SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;
use reqwest;
use snafu::{ensure, ResultExt, Snafu};
use tokio;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to fetch block hash: {}", source))]
    FetchError { source: reqwest::Error, backtrace: Backtrace },
    #[snafu(display("Wrong sized block hash: len:{} hex:{}", hash.len(), hex::encode(hash)))]
    WrongSizeError { hash: Vec<u8>, backtrace: Backtrace },
    #[snafu(display("Failed to decode block hash: {}", source))]
    DecodeError { source: hex::FromHexError, backtrace: Backtrace },
}

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let block_hash = fetch_latest_hash().await?;
    println!("Block hash: {}", hex::encode(&block_hash));

    // last 8 bytes of the block hash
    let seed = u64::from_le_bytes(block_hash[24..].try_into().unwrap());
    println!("Seed: 0x{}", hex::encode(seed.to_le_bytes()));

    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut names = vec![
        "alecthomas",
        "brad",
        "deniseli",
        "gak",
        "jonathanj-square",
        "juho",
        "matt2e",
        "safeer",
        "stuartwdouglas",
        "wesbillman",
        "worstell",
    ];
    println!("Original: {:?}", names);
    names.shuffle(&mut rng);
    println!("\nShuffled:\n{}", names.join("\n"));
    Ok(())
}

async fn fetch_latest_hash() -> Result<Vec<u8>, Error> {
    let url = "https://blockchain.info/q/latesthash";
    let hash_string = reqwest::get(url)
        .await
        .context(FetchSnafu)?
        .text()
        .await
        .context(FetchSnafu)?;

    let hash = hex::decode(&hash_string[..64]).context(DecodeSnafu)?;
    ensure!(hash.len() == 32, WrongSizeSnafu { hash });

    Ok(hash)
}
