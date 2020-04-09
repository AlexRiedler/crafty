use std::env;
use std::fs;
use std::io;
use std::io::Write;
mod scanner;
use scanner::token::Token;

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

    run(&contents);
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .ok()
            .expect("Failed to read line");

        run(&line);
    }
}


fn run(source: &String) {
    let tokens: Vec<Token> = scanner::scan_tokens(source);

    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

fn error(line: i64, message: &String) {
    report(line, &"".to_string(), message);
}

fn report(line: i64, location: &String, message: &String) {
    println!("[line {}] Error {}: {}", line, location, message);
}
