use crate::Visitor;
use crate::parser::Expr;
use crate::scanner::token::TokenType;

pub struct RuntimeError {
    pub message: String,
}

#[derive(Debug)]
pub enum Operator {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Add,
    Subtract,
    Divide,
    Multiply
}

pub struct ExprEvaluator;

impl Visitor<Result<f64, RuntimeError>> for ExprEvaluator {
    fn visit_expr(&mut self, e: &Expr) -> Result<f64, RuntimeError> {
        match &*e {
            Expr::BoolLiteral(b) => Ok(bool_to_f64(*b)),
            Expr::StringLiteral(n) => Err(RuntimeError{message: format!("Found unexpected string literal {}", n)}),
            Expr::NumberLiteral(n) => Ok(n.parse::<f64>().unwrap()),
            Expr::Operator(token_type, n) => Err(RuntimeError{message: format!("Received operator {:?} {} outside of expression", token_type, n)}),
            Expr::Unary(ref operator, ref rhs) => 
                match operator_from_expression(operator)? {
                    Operator::Bang => Ok(self.visit_expr(rhs)?),
                    Operator::Subtract => Ok(-self.visit_expr(rhs)?),
                    op => Err(RuntimeError{message: format!("Invalid unary opeartor {:?}", op)}),
                },
            Expr::Binary(ref lhs, ref operator, ref rhs) =>
                match operator_from_expression(operator)? {
                    Operator::BangEqual => Ok(bool_to_f64(self.visit_expr(lhs)? != self.visit_expr(rhs)?)),
                    Operator::EqualEqual => Ok(bool_to_f64(self.visit_expr(lhs)? != self.visit_expr(rhs)?)),
                    Operator::Greater => Ok(bool_to_f64(self.visit_expr(lhs)? > self.visit_expr(rhs)?)),
                    Operator::GreaterEqual => Ok(bool_to_f64(self.visit_expr(lhs)? >= self.visit_expr(rhs)?)),
                    Operator::Less => Ok(bool_to_f64(self.visit_expr(lhs)? < self.visit_expr(rhs)?)),
                    Operator::LessEqual => Ok(bool_to_f64(self.visit_expr(lhs)? <= self.visit_expr(rhs)?)),
                    Operator::Add => Ok(self.visit_expr(lhs)? + self.visit_expr(rhs)?),
                    Operator::Subtract => Ok(self.visit_expr(lhs)? - self.visit_expr(rhs)?),
                    Operator::Multiply => Ok(self.visit_expr(lhs)? * self.visit_expr(rhs)?),
                    Operator::Divide => Ok(self.visit_expr(lhs)? / self.visit_expr(rhs)?),
                    op => Err(RuntimeError{message: format!("Invalid inline opeartor {:?}", op)}),
                },
            Expr::Grouping(ref expr) => self.visit_expr(expr),
        }
    }
}

fn bool_to_f64(boolean: bool) -> f64 {
    if boolean { 1.0 } else { 0.0 }
}

// DEFER: this should probably be part of parsing?
fn operator_from_expression(e: &Expr) -> Result<Operator, RuntimeError> {
    match &e {
        Expr::Operator(token_type, _string) =>
            match token_type {
                TokenType::Bang => Ok(Operator::Bang),
                TokenType::BangEqual => Ok(Operator::BangEqual),
                TokenType::Equal => Ok(Operator::Equal),
                TokenType::EqualEqual => Ok(Operator::EqualEqual),
                TokenType::Greater => Ok(Operator::Greater),
                TokenType::GreaterEqual => Ok(Operator::GreaterEqual),
                TokenType::Less => Ok(Operator::Less),
                TokenType::LessEqual => Ok(Operator::LessEqual),
                TokenType::Minus => Ok(Operator::Subtract),
                TokenType::Plus => Ok(Operator::Add),
                TokenType::Star => Ok(Operator::Multiply),
                TokenType::Slash => Ok(Operator::Divide),
                _ => Err(RuntimeError{message: format!("Received unknown operator {:?}", token_type)})
            }
        _ => Err(RuntimeError{message: format!("Received non-operator expression in operator expression field")}),
    }
}
