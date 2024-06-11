use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: String
}

fn main() {
    println!("{}", Args::parse().input);
}
