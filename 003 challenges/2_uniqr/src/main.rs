use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    filename: String,
    #[arg(short('o'), long("option"))]
    option: String
}

fn main() {
    let args = Args::parse();
    if let Err(err) = run(&args.filename, &args.option) {
        println!("{err}: {}", Args::parse().filename);
        return;
    }   
}

fn run(filename: &str, option: &str) -> Result<(), std::io::Error> {
    let reader = BufReader::new(File::open(filename)?);
    
    let mut previous: String = String::new();
    let mut count: u32 = 1;
    
    match option {
        "count" =>  {
            for line in reader.lines() {
                let current_line = line.unwrap(); 
                if &current_line != &previous {
                    match count {
                        0 => {
                            print!("");
                        },
                        1 => {
                            println!("1 {}", previous);
                        },
                        _ => {
                            println!("{} {}", count, previous);
                            count = 1;
                        }
                    }
                } else {
                    count += 1
                }
                previous = current_line.clone();
            }
            println!("{} {}", count, previous);
        },
        "rep" => {
            for line in reader.lines() {
                let current_line = line.unwrap(); 
                if &current_line != &previous {
                    match count {
                        0 => {
                            print!("");
                        },
                        _ => match previous.as_str() {
                            "" => {
                                print!("")
                            },
                            _ => {
                                println!("{}", previous);
                                count = 1;
                            }
                        }
                    }
                } else {
                    count += 1
                }
                previous = current_line.clone();
            }
            println!("{}", previous);
        },
        _ => {
            let msg = r#"
You entered the invalid option

Option:
- count: Count the unique sentences
- rep: Display only repeated sentences

uniqr [filename] --option [option]
            "#;

            println!("{}", msg);
        }    
    }

    Ok(())
}

