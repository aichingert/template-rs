use template_rs::*;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Give the year and day when executing the programm");
        std::process::exit(1);
    }

    let mut template = Template::new(args[1].clone(), args[2].clone());
    
}