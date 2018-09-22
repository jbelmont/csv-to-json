use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader,Write,BufWriter};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    read_file(file);
}

fn read_file(filename: &str) {
    let f = File::open(filename).expect("file not found");

    let file = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        if num == 0 {
            let l = line.unwrap();
            let v: Vec<&str> = l.split(' ').collect();
            println!("{:?}", v);
        }
    }
}
