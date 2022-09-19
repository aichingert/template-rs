use template_rs::*;
use std::path::Path;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() <= 2 {
        println!("Give the year and day when executing the programm");
        println!("=================================================\n");

        std::process::exit(1);
    } else {
        let year: Result<i32, std::num::ParseIntError> = args[1].parse::<i32>();
        let day: Result<i32, std::num::ParseIntError> = args[2].parse::<i32>();

        match year {
            Ok(n) => {
                if n < 2015 || n > 2022 {
                    println!("Enter a valid year! 2015-2022");
                    std::process::exit(1);
                }
            },
            Err(_) => {
                println!("Enter a valid year! 2015-2022");
                std::process::exit(1);
            }
        }

        match day {
            Ok(n) => {
                if n < 1 || n > 25 {
                    println!("Enter a valid day! 1-25");
                    std::process::exit(1);
                }
            },
            Err(_) => {
                println!("Enter a valid day! 1-25");
                std::process::exit(1);
            }
        }
    }

    if args[2].len() != 2 {
        println!("Invalid day add 0 at the beginning if day <= 9");
        println!("==============================================\n");
        std::process::exit(1);
    }

    let path: String = format!("../aoc-rs/aoc/src/bin/aoc{}/aoc{}_{}.rs", &args[1], &args[1], &args[2]);

    if Path::exists(Path::new(&path)) {
        println!("File already exists: aoc{}_{}.rs",&args[1], &args[2]);
        std::process::exit(1);
    }

    let mut template = Template::new(args[1].clone(), args[2].clone());
    println!("Created file aoc{}_{} ==========> [20.00 %]", args[1], args[2]);

    template.write_template().expect("couldn't edit file");
    println!("Added content to file ============> [40.00 %]");

    template.update_mod().expect("couldn't edit mod.rs");
    println!("Added day to mod.rs - finished ===> [60.00 %]");

    template.add_txt().expect("couldn't create input txt");
    println!("Created input txt ================> [80.00 %]");

    template.update_readme().expect("couldn't edit/create README.md");
    println!("Added day to README.md ===========> [100.0 %]");
}