use std::fs::{File, read_to_string};
use std::io::Write;
use std::fmt;

pub struct FileExistsError {
    path: String
}

impl fmt::Display for FileExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The file [\"{}\"] already exists", self.path)
    }
}

impl fmt::Debug for FileExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) 
    }
}

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

    fn sorted_insert(&self, is_added: bool, current_day: i32, day: i32, to_add: String, content: &mut Vec<String>, idx: usize) -> bool {
        let mut did_add: bool = is_added;
        
        if current_day < day && !did_add {
            content.insert(idx, to_add.clone());
            did_add = true;
        } else if current_day == day {
            did_add = true;
        }

        if !did_add && content[idx+1] == "" {
            content[idx].push('\n');
            content.insert(idx+1, to_add);
        } else {
            content[idx].push('\n');
        }       

        did_add
    }
    
    pub fn write_template(&mut self) -> std::io::Result<()> {
        let name = format!("Aoc{}_{}", &self.year, &self.day);
        let year: &String = &self.year;
        let day: &String = &self.day;

        write!(self.file, "pub struct {name} {{
    d: Vec<i32>
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
        self.d = aoc::(\"input/{year}/{day}.txt\");
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
        let mut file: File = File::create(&path)?;
        let current_day: i32 = self.day.parse().unwrap();
        let mut mod_is_added: bool = false;
        let mut use_is_added: bool = false;
        let mut new_is_added: bool = false;
        let mut idx: usize = 0;
        content[0].push('\n');

        for i in 1..content.len() {
            let line: Vec<&str> = content[i].split(' ').collect();

            match line[0] {
                "mod" => {
                    let day: i32 = line[1][8..=9].parse::<i32>().unwrap();
 
                    mod_is_added = self.sorted_insert(mod_is_added, current_day, day, format!("mod aoc{}_{};", self.year, self.day), &mut content, i);
                },
                "use" => {
                    let day: i32 = line[1][8..=9].parse::<i32>().unwrap();

                    use_is_added = self.sorted_insert(use_is_added, current_day, day, format!("use aoc{}_{}::*;", self.year, self.day), &mut content, i);
                },
                "pub" => {
                    idx = i;
                    break;
                }
                _ => {
                    content[i].push('\n');
                }
            }
        }

        for i in idx..content.len() {
            let line: Vec<&str> = content[i].split(' ').collect();

            if line.len() > 6 && line[6].len() > 5 {
                match line[4] {
                    "let" => {
                        let day: i32 = line[6][4..=5].parse().unwrap();
                        new_is_added = self.sorted_insert(new_is_added, current_day, day, format!("    let mut day_{} = Aoc{}_{}::new();", self.day, self. year, self.day), &mut content, i);
                        println!("{day} {new_is_added}");
                    }
                    _ => {
                        let day: i32 = line[6][4..=5].parse().unwrap();
                        println!("{day} {new_is_added}");
                    }
                }
            } else {
                content[i].push('\n');
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

    pub fn add_txt(&self) -> Result<(), FileExistsError> {
        let path: String = format!("../aoc-rs/aoc/input/{}/{}.txt", self.year, self.day);
        
        if std::path::Path::new(&path).exists() {
            return Err(FileExistsError {
                path
            })
        }

        File::create(path).expect("unable to create file");

        Ok(())
    }

    pub fn update_readme(&self) -> std::io::Result<()> {
        let mut path: String = "../aoc-rs/aoc/src/bin/aoc".to_owned() + &self.year + "/README.md";
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
        let check = if &day[0..=0] == "0" { day.replace("0", " ") } else { day.to_string() };

        let line: String = format!("| {} | **[name](https://adventofcode.com/{year}/day/{day})** | [day {}](/aoc/src/bin/aoc{year}/aoc{year}_{day}.rs) |\r\n", check, check);

        
        content.push(line);

        let days: String = (content.len()-3).to_string();
        let mut file_content = content.into_iter().collect::<String>();
        write!(file, "{file_content}")?;

        path = "../aoc-rs/README.md".to_string();
        content = read_to_string(&path)
            .expect("unable to open file")
            .lines()
            .map( | l | l.to_string())
            .collect();
        file = File::create(&path)?;

        let mut line: Vec<&str> = content[11 - (year.parse::<usize>().unwrap() - 2015) as usize].split(" ").collect();
        line[4] = &days;
        content[11 - (year.parse::<usize>().unwrap() - 2015) as usize] = line.into_iter().collect::<String>();

        for i in 0..content.len() {
            content[i].push_str("\r\n");
        }
        
        file_content =  content.into_iter().collect::<String>();
        write!(file, "{file_content}")?;
        
        Ok(())
    }
}

fn create(year: &String, day: &String) -> std::io::Result<File> {
    let file = File::create("../aoc-rs/aoc/src/bin/aoc".to_owned() + year + "/" + "aoc" + year + "_" + day + ".rs")?;
    Ok(file)
}
