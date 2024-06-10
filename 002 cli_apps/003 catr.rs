use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    filename: String
}

fn main() {
    let filename = Args::parse().filename;
    if let Err(err) = run(&filename) {
        println!("{err}: {filename}");
        return;
    }   
}

fn run(filename: &str) -> Result<(), std::io::Error> {
    let reader = BufReader::new(File::open(filename)?);
    for line in reader.lines() {
        let line = line?;
        println!("{line}")
    }
    Ok(())
}

