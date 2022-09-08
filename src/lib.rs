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
    
    pub fn write_template(&mut self) -> std::io::Result<()> {
        let name = format!("Aoc{}_{}", &self.year, &self.day);

        write!(self.file, "use aoc::*;

pub struct {name} {{
    d: Vec<_>
}}
        
impl {name} {{
    pub fn new() -> Self {{
        Self {{ d: vec![] }}
    }}
}}
        
impl crate::Solution for {name} {{
    fn name(&self) -> (usize, usize) {{
        ({}, {})
    }}
        
    fn parse(&mut self) {{
        self.d;
    }}
        
    fn part1(&mut self) -> Vec<String> {{
        crate::output(\"\")
    }}
        
    fn part2(&mut self) -> Vec<String> {{
        crate::output(\"\")
    }}
}}", &self.year, &self.day)?;

        Ok(())
    }
}

fn create(year: &String, day: &String) -> std::io::Result<File> {
    let file = File::create("../aoc-rs/aoc/src/bin/aoc".to_owned() + year + "/" + "aoc" + year + "_" + day + ".rs")?;
    Ok(file)
}