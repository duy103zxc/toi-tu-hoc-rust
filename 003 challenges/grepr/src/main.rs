use clap::Parser;
use std::io::{BufRead, BufReader};
use std::fs::{self, DirEntry, File};

#[derive(Debug, Parser)]
struct Grepmin {
    #[arg(default_value = "", short('m'))]
    mode: String,
    #[arg(default_value = "", short('k'))]
    keyword: String,
    #[arg(default_value = "", short('f'))]
    file: String
}

fn main() {
    let args = Grepmin::parse();
    grep(&args.mode, &args.file, &args.keyword).unwrap();
}

fn grep(mode: &str, file: &str, key: &str) -> Result<(), std::io::Error> {
    match mode {
        "w" => word_mode(file, key),
        "d" => {
            let paths = fs::read_dir(file).unwrap(); 
            for path in paths {
                dir_mode(&path.unwrap(), key).unwrap();
            }
            Ok(())
        },
        _ => {
            println!("Invalid option");
            Ok(())
        }
    }
}

fn word_mode(file: &str, key: &str) -> Result<(), std::io::Error> {
    let reader = BufReader::new(File::open(&file)?);
    
    for line in reader.lines() {
        let line = line?;
        if line.contains(key) {
            println!("{}", line);
        } else {
            print!("");
        }
    }
    Ok(())
}


fn dir_mode(given_dir: &DirEntry, keyword: &str) -> Result<(), std::io::Error> {
    match given_dir.path().is_dir() {
        true => {
            given_dir.path()
            .read_dir()
            .unwrap().for_each(|f| find_inside_dir(&f.unwrap(), keyword));
            Ok(())
        },
        false => {
            find_inside_dir(given_dir, keyword);
            Ok(())
        }
    } 
}

fn find_inside_dir(x: &DirEntry, k: &str) {
    if x.path().into_os_string().into_string().unwrap().contains(k) {
        println!("{}", x.path().display());         
    } else {
        print!("");
    }
}