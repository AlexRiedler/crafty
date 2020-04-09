use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: crafty [script]");
    } else if args.len() == 2 {
        let filename = &args[1];
        run_file(filename);
    } else {
        run_prompt();
    }
}

fn run_file(filename: &String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    run(contents);
}

fn run_prompt() {
    println!("Prompt not implemented yet!");
}


fn run(contents: String) {
}
