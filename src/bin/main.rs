use template_rs::*;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
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

    let mut template = Template::new(args[1].clone(), args[2].clone());
    println!("Created file aoc{}_{} ======================> [33.33 %]", args[1], args[2]);

    template.write_template().expect("couldn't edit file");
    println!("Added content to file ========================> [66.66 %]");

    template.update_mod();
    println!("Succesfully added day to mod.rs - finished ===> [100.0 %]");
}