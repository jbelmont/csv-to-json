extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    read_file(file);
}

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone)]
struct Demographic {
    jurisdiction_name: usize,
    count_participants: usize,
    count_female: usize,
    percent_female: f64,
    count_male: usize,
    percent_male: f64,
    count_gender_unknown: usize,
    percent_gender_unknown: f64,
}

impl Demographic {
    fn new() -> Self {
        Default::default()
    }
}

fn read_file(filename: &str) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut demographic: Demographic = Demographic::new();
    let mut keys = vec![String::from("")];
    let mut list: Vec<Demographic> = vec![];
    for (num, line) in file.lines().enumerate() {
        if num == 0 {
            let l = line.unwrap();
            let v = l.split(',')
                .map(|field| field.to_lowercase().replace(" ", "_"))
                .collect();
            keys = v;
        } else {
            let l = line.unwrap();
            let values: Vec<String> = l.split(',')
                .map(|val| val.to_string())
                .collect();

            for item in keys.iter() {
                if item == "jurisdiction_name" {
                    demographic.jurisdiction_name = values[0].parse().unwrap();
                } else if item == "count_participants" {
                    demographic.count_participants = values[1].parse().unwrap();
                } else if item == "count_female" {
                    demographic.count_female = values[2].parse().unwrap();
                } else if item == "percent_female" {
                    demographic.percent_female = values[3].parse().unwrap();
                } else if item == "count_male" {
                    demographic.count_male = values[4].parse().unwrap();
                } else if item == "percent_male" {
                    demographic.percent_male = values[5].parse().unwrap();
                } else if item == "count_gender_unknown" {
                    demographic.count_gender_unknown = values[6].parse().unwrap();
                } else if item == "percent_gender_unknown" {
                    demographic.percent_gender_unknown = values[7].parse().unwrap();
                }
            }
            list.push(demographic);
        }
    }

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&list).expect("could not serialize demographics");

    // Print, write to a file, or send to an HTTP server.
    println!("{:?}", j);
}
