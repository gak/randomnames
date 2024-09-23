#![feature(error_generic_member_access)]

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;
use reqwest;
use sha2::Digest;
use sha2::Sha256;
use snafu::{ensure, ResultExt, Snafu};
use std::backtrace::Backtrace;
use tokio;

#[derive(Debug, Snafu)]
enum Error {}

const NAMES: &[&str] = &[
    "abrooks",
    "alecthomas",
    "brad",
    "deniseli",
    "gak",
    "jonathanj",
    "juho",
    "matt2e",
    "safeer",
    "stuartwdouglas",
    "tlongwell",
    "tom",
    "wesbillman",
    "worstell",
];

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Error> {
    for days_in_the_future in 0..=7 {
        let now = Utc::now();
        let day = now + Duration::days(days_in_the_future);
        let names = get_names(&day);

        let names_str = names.join(", ");
        println!("{}Z: {names_str}", day.format("%Y-%m-%d"));
    }

    Ok(())
}

fn get_names(now: &DateTime<Utc>) -> Vec<&str> {
    let seed = seed_from_date(now);
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut names = NAMES.to_vec();
    names.shuffle(&mut rng);
    names
}

fn seed_from_date(date: &DateTime<Utc>) -> u64 {
    let day = date.format("%Y-%m-%d").to_string();
    let mut hasher = Sha256::default();
    hasher.update(day.as_bytes());
    let bytes = hasher.finalize();
    u64::from_ne_bytes(bytes[0..8].try_into().unwrap())
}
