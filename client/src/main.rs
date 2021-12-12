extern crate clap;

use clap::{App, Arg};
use client::Call;

pub(crate) mod client;
pub mod command;

#[tokio::main]
async fn main() {
    let matches = App::new("Wasmship, a ship to wasm.")
        .version("0.1.0")
        .about("Yang <zifeng.1024@gmail.com>")
        .arg(
            Arg::new("run")
                .long("run")
                .value_name("MODULE")
                .takes_value(true),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .value_name("MODULE")
                .possible_values(["MODULE", "INSTANCE"])
                .takes_value(true),
        )
        .get_matches();

    let mut client = client::Client::init();
    client.call().await;
}
