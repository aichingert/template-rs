use template_rs::*;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Give the year and day when executing the programm");
        println!("=================================================\n");

        std::process::exit(1);
    }

    let mut template = Template::new(args[1].clone(), args[2].clone());
    println!("Created file aoc{}_{} ======================> [33.33 %]", args[1], args[2]);

    template.write_template().expect("couldn't edit file");
    println!("Added content to file ========================> [66.66 %]");

    template.update_mod();
    println!("Succesfully added day to mod.rs - finished ===> [100.0 %]");
}