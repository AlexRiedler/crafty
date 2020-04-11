use std::env;
use std::fs;
use std::io;
use std::io::Write;
mod scanner;
use scanner::token::Token;

mod parser;

fn main() {
    /*
    let mut printer = AstPrinter{};

    let string = printer.visit_expr(
        &Expr::Binary(
            Box::new(
                Expr::Literal(Token {
                    token_type: scanner::token::TokenType::Identifier,
                    lexeme: String::from("45"),
                    line_number: 0,
                })
            ),
            Box::new(
                Token {
                    token_type: scanner::token::TokenType::Identifier,
                    lexeme: String::from("+"),
                    line_number: 0,
                }
            ),
            Box::new(
                Expr::Literal(Token {
                    token_type: scanner::token::TokenType::Identifier,
                    lexeme: String::from("45"),
                    line_number: 0,
                })
            ),
        )
   );

   println!("{}", string)
   */

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

pub enum Expr {
    Literal(Token),
    Unary(Box<Token>, Box<Expr>),
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
}

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        match &*e {
            Expr::Literal(n) => n.lexeme.clone(),
            Expr::Unary(ref n, ref rhs) => format!("({} {})", n.lexeme, self.visit_expr(rhs)),
            Expr::Binary(ref lhs, ref operator, ref rhs) => format!("({} {} {})", operator.lexeme, self.visit_expr(lhs), self.visit_expr(rhs)),
            Expr::Grouping(ref expr) => format!("({})", self.visit_expr(expr)),
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
}

fn error(line: i64, message: &String) {
    report(line, &"".to_string(), message);
}

fn report(line: i64, location: &String, message: &String) {
    println!("[line {}] Error {}: {}", line, location, message);
}
