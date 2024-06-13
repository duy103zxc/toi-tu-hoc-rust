use std::fs::{self, DirEntry};
use clap::Parser;

#[derive(Debug, Parser)]
struct Finder {
    #[arg(default_value = "./", short('f'))]
    path: String,
    #[arg(short('k'), long("key"))]
    keyword: String,
}



fn main() {
    let finder = Finder::parse();
    ls(&finder.path, &finder.keyword);
}


fn ls(path: &str, word: &str) {
    let paths = fs::read_dir(path).unwrap();
    // let tmp: String = String::from("tmp.txt");
    for path in paths {
        check_dir(&path.unwrap(), word);
    }
}


fn check_dir(given_dir: &DirEntry, keyword: &str) {
    match given_dir.path().is_dir() {
        true => {
            given_dir.path()
            .read_dir()
            .unwrap().for_each(|f| find_inside_dir(&f.unwrap(), keyword))
        },
        false => {
            find_inside_dir(given_dir, keyword)
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

