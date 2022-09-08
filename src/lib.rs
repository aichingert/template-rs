use std::fs::{File};
use std::io::Write;

pub struct Template {
    file: File,
    year: String,
    day: String
}

impl Template {
    pub fn new(year: String, day: String) -> Self {
        Self {
            file: create(&year, &day).unwrap(),
            year,
            day
        }
    }
}

fn create(year: &String, day: &String) -> std::io::Result<File> {
    let file = File::create("../aoc-rs/aoc/src/bin/aoc".to_owned() + year + "/" + "aoc" + year + "_" + day + ".rs")?;
    Ok(file)
}