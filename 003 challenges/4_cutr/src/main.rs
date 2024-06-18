use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;


#[derive(Debug, Parser)]
struct Cutter {
    #[arg(default_value = "-", short('f'))]
    filename: String,
    #[arg(default_value = "byte", short('m'), long("mode"))]
    mode: String,
    #[arg(default_value = "", short('c'))]
    opt: String,
}



fn main() {
    let cutter = Cutter::parse();
    run(&cutter.filename, &cutter.mode, cutter.opt);
}

fn run(filename: &str, cut_mode: &str, c: String) {
    let reader = BufReader::new(File::open(filename).unwrap());
    let char_list: Vec<u32> = c.split(",").into_iter().for_each(|f| f.to_string()).;
    match cut_mode {
        "byte" => {
            for line in reader.lines() {
                let line = line.unwrap();
                println!("{}", line);
            }
        },
        "char" => {

        },
        _ => {

        }
    }
}



