use std::{fs::File, io::{BufRead, BufReader, Read, Seek}};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    filename: String,
    #[arg(default_value("-"), long("type"))]
    choice: String
}

fn main() {
    let args = Args::parse();
    if let Err(err) = count(&args.filename, &args.choice) {
        println!("{err}");
        std::process::exit(1)
    }   
}

fn count(filename: &str, option: &str) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(File::open(&filename)?);

    let line_counter = reader.by_ref().lines().count();
    reader.rewind().unwrap();
    let word_counter = reader.by_ref().lines().fold(0, |f, x| f + x.unwrap_or_default().split(" ").into_iter().count());

    match option {
        "word" => {
            println!("word count: {}", word_counter)
        },
        "line" => {
            println!("line count: {}", line_counter)
        },
        _ => println!("Word: {}, Line: {}", word_counter, line_counter)
    }
    Ok(())
}

