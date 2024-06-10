use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    filename: String,
    
    #[arg(short('n'), long("number"))]
    num_line: u64
}

fn main() {
    // let filename = Args::parse().filename;
    if let Err(err) = run(Args::parse()) {
        println!("{err}: {}", Args::parse().filename);
        return;
    }   
}

fn run(args: Args) -> Result<(), std::io::Error> {
    let reader = BufReader::new(File::open(args.filename)?);
    let mut total_line: u64 = 1;
    for line in reader.lines() {
        let line = line?;
        if total_line <= args.num_line {
            println!("{line}");
        } else {
            std::process::exit(1);
        }
        total_line += 1;
    }
    Ok(())
}

