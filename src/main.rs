use std::env;
use std::fs;
use std::io;
use std::io::Write;
mod scanner;
use scanner::token::Token;

mod parser;
use parser::Expr;
use parser::Parser;

use itertools::multipeek;

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

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
}

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        match &*e {
            Expr::BoolLiteral(b) => format!("{}", b),
            Expr::StringLiteral(n) => n.to_string(),
            Expr::NumberLiteral(n) => n.to_string(),
            Expr::Operator(n) => n.to_string(),
            Expr::Unary(ref operator, ref rhs) => format!("({} {})", self.visit_expr(operator), self.visit_expr(rhs)),
            Expr::Binary(ref lhs, ref operator, ref rhs) => format!("({} {} {})", self.visit_expr(operator), self.visit_expr(lhs), self.visit_expr(rhs)),
        }
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

    let mut parser = Parser{
        iter: multipeek(tokens.iter()),
        current: None,
        previous: None,
    };
    let expr = parser.expression();
    match expr {
        Ok(result) => {
            let mut printer = AstPrinter{};
            let string = printer.visit_expr(&result);
            println!("{}", string);
        },
        Err(_error) => {
            println!("Error parsing");
        }
    }
}

fn error(line: i64, message: &String) {
    report(line, &"".to_string(), message);
}

fn report(line: i64, location: &String, message: &String) {
    println!("[line {}] Error {}: {}", line, location, message);
}
