use std::time::Duration;
use std::time::Instant;
use std::thread;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long("mode"), short('m'))]
    mode: String,
    #[arg(long("work"), short('w'), default_value = "30")]
    work: u64,
    #[arg(long("rest"), short('r'),  default_value = "6")]
    rest: u64
}

fn main() {
    let args = Args::parse();
    timer(&args.mode, args.work, args.rest);
}



fn timer(mode: &str, work: u64, rest: u64) {
    match mode {
        "pomo" => {
            let start = Instant::now();
            thread::sleep(Duration::from_secs(30*60));
            let duration = start.elapsed();
            println!("Done {:?}, resting", duration);
            thread::sleep(Duration::from_secs(6*60));
        },
        "flowmo" => {
            let start = Instant::now();
            thread::sleep(Duration::from_secs(work*60));
            let duration = start.elapsed();
            println!("Done {:?}, resting", duration);
            thread::sleep(Duration::from_secs(rest*60));
        },
        _ => {
            println!("Mode not found")
        }
    }
    
}
