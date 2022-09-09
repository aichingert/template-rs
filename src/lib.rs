use std::fs::{File, read_to_string};
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
        let year: &String = &self.year;
        let day: &String = &self.day;

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
        ({year}, {day})
    }}
        
    fn parse(&mut self) {{
        self.d = x(\"input/{year}/{day}.txt\");
    }}
        
    fn part1(&mut self) -> Vec<String> {{
        crate::output(\"\")
    }}
        
    fn part2(&mut self) -> Vec<String> {{
        crate::output(\"\")
    }}
}}")?;

        Ok(())
    }

    pub fn update_mod(&mut self) -> std::io::Result<()> {
        let path: String = "../aoc-rs/aoc/src/bin/aoc".to_owned() + &self.year + "/mod.rs";
        let mut content: Vec<String> = read_to_string(&path)
            .expect("unable to open file")
            .lines()
            .map( | l | l.to_string())
            .collect();
        let mut new_line: u8 = 0;
        let mut file: File = File::create(&path)?;
        
        for i in 0..content.len() {
            if content[i] == "" {
                new_line += 1;
            }

            match new_line {
                2 => {
                    let element: String = format!("mod aoc{}_{};\r\n", &self.year, &self.day);
                    content.insert(i, element);
                },
                4 => {
                    let element: String = format!("use aoc{}_{}::*;\r\n", &self.year, &self.day);
                    content.insert(i, element);
                },
                6 => {
                    let element: String = format!("\tlet mut day_{} = Aoc{}_{}::new();\r\n", &self.day, &self.year, &self.day);
                    content.insert(i, element);
                },
                7 => {
                    content[i].push_str("\r\n");
                    content[i + 2].push_str(&format!(", &mut day_{}", &self.day));
                    new_line += 1;
                }
                _ => {
                    content[i].push_str("\r\n");
                }
            }
        }

        let len = content.len();

        for i in 0..3 {
            content[len - i - 1].push_str("\r\n");
        }

        let file_content = content.into_iter().collect::<String>();
        write!(file, "{file_content}")?;

        Ok(())
    }

    pub fn add_txt(&self) -> std::io::Result<()> {
        let path: String = format!("../aoc-rs/aoc/input/{}/{}.txt", self.year, self.day);
        File::create(path)?;

        Ok(())
    }

    pub fn update_readme(&self) -> std::io::Result<()> {
        let path: String = "../aoc-rs/aoc/src/bin/aoc".to_owned() + &self.year + "/README.md";
        let mut content: Vec<String> = read_to_string(&path)
            .expect("unable to open file")
            .lines()
            .map( | l | l.to_string())
            .collect();
        let mut file: File = File::create(&path)?;

        for i in 0..content.len() {
            content[i].push_str("\r\n");
        }

        let year: &String = &self.year;
        let day: &String = &self.day;

        let line: String = format!("| {day} | **[name](https://adventofcode.com/{year}/day/{day})** | [day {}](/aoc/src/bin/aoc{year}/aoc{year}_{day}.rs) |\r\n", if &day[0..=0] == "0" { day.replace("0", " ") } else { day.to_string() });

        content.push(line);

        let file_content = content.into_iter().collect::<String>();
        write!(file, "{file_content}")?;

        Ok(())
    }
}

fn create(year: &String, day: &String) -> std::io::Result<File> {
    let file = File::create("../aoc-rs/aoc/src/bin/aoc".to_owned() + year + "/" + "aoc" + year + "_" + day + ".rs")?;
    Ok(file)
}