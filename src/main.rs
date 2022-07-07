use clap::Parser;
use crate::lib::five_results;

pub mod lib;
/// Simple program to generate passwords
#[derive(Parser, Debug)]
#[clap(author="Author: Makariy Balashov, tg: @ya-makariy", version, about, long_about = None)]
struct Args {
    /// If present, create password of defined length. Default value is 8.
    #[clap(short, long)]
    length: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let l: usize = match args.length {
        None => 8,
        Some(i) => i,
    };

    println!("Generated passwords:");
    for pass in five_results(l) {
        println!("{}", pass);
    }
}
