use std::env;
use std::fs;
use std::io;
use std::io::Write;

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
    let tokens: Vec<&str> = source.split(' ').collect();

    for token in tokens.iter() {
        println!("{}", token);
    }
}
