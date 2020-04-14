use std::env;
use std::fs;
use std::io;
use std::io::Write;
mod scanner;
use scanner::token::Token;
use scanner::token::TokenType;

mod parser;
use parser::Parser;
use parser::ParseError;

mod runtime;
use runtime::ExprEvaluator;

mod printer;
use printer::AstPrinter;

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
    let tokens: Vec<Token> =
        scanner::scan_tokens(source)
        .into_iter()
        .filter(|tok| tok.token_type != TokenType::Whitespace)
        .inspect(|tok| println!("{:?}", tok))
        .collect();

    let mut parser = Parser{
        iter: tokens.iter().peekable(),
        current: None,
        previous: None,
    };
    match parser.parse() {
        Ok(expr) => {
            AstPrinter{}.print(&expr);
            ExprEvaluator{}.evaluate(&expr);
        },
        Err(ParseError{message}) => {
            println!("Error parsing: {}", message);
        }
    }
}
