use std::collections::HashMap;
use crate::parser::Expr;
use crate::parser::Statement;
use crate::parser::Visitor;
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

#[derive(Debug, Clone)]
pub enum Object {
    Nil(),
    Float(f64),
    Integer(i64),
    Boolean(bool),
    StringLiteral(String),
}

pub fn build_interpreter() -> ExprEvaluator {
    let mut environments = Vec::new();
    environments.push(Environment{
        values: HashMap::new()
    });

    ExprEvaluator{
        environments
    }
}

pub struct Environment {
    pub values: HashMap<String, Object>,
}

impl Environment {
}

pub struct ExprEvaluator {
    environments: Vec<Environment>,
}

impl ExprEvaluator {
    pub fn interpret(&mut self, statements: &Vec<Statement>) {
        for statement in statements.iter() {
            let result = self.visit_statement(statement);
            match result {
                Ok(_object) => {},
                Err(RuntimeError{message}) => {
                    println!("Error evaluating: {}", message);
                }
            }
        }
    }

    fn execute_block(&mut self, statements: &Vec<Statement>) -> Result<Object, RuntimeError> {
        self.environments.push(Environment{
            values: HashMap::new()
        });

        let mut last_value = Object::Nil();

        for statement in statements.iter() {
            match self.execute(statement) {
                Ok(object) => last_value = object,
                error => {
                    self.environments.pop();
                    return error;
                }
            }
        }

        self.environments.pop();
        Ok(last_value)
    }

    fn execute(&mut self, statement: &Statement) -> Result<Object, RuntimeError> {
        self.visit_statement(statement)
    }

    pub fn define_variable(&mut self, name: String, object: Object) {
        match self.environments.last_mut() {
            Some(environment) => environment.values.insert(name, object),
            None => None // TODO: probably should error out, no environments present
        };
    }

    pub fn get_variable(&self, name: &String) -> Result<Object, RuntimeError> {
        for environment in self.environments.iter().rev() {
            match environment.values.get(name) {
                Some(object) => return Ok(object.clone()),
                None => {}
            }
        }
        return Err(RuntimeError{message: format!("Undefined variable '{}'.", name)});
    }

    pub fn assign_variable(&mut self, name: String, object: Object) -> Result<Object, RuntimeError> {
        for environment in self.environments.iter_mut().rev() {
            match environment.values.get(&name) {
                Some(_) => {
                    environment.values.insert(name, object.clone());
                    return Ok(object);
                },
                None => {}
            }
        }
        return Err(RuntimeError{message: format!("Undefined variable '{}'.", name)});
    }
}

impl Visitor<Result<Object, RuntimeError>> for ExprEvaluator {
    fn visit_expr(&mut self, e: &Expr) -> Result<Object, RuntimeError> {
        match &*e {
            Expr::Assign(token, ref expr) => {
                let result = self.visit_expr(expr)?;
                self.assign_variable(token.lexeme.to_string(), result.clone())?;
                Ok(result)
            },
            Expr::Variable(token) => self.get_variable(&token.lexeme),
            Expr::BoolLiteral(b) => Ok(Object::Boolean(*b)),
            Expr::StringLiteral(n) => Ok(Object::StringLiteral(n.to_string())),
            Expr::IntegerLiteral(n) => Ok(Object::Integer(n.parse::<i64>().unwrap())),
            Expr::FloatLiteral(n) => Ok(Object::Float(n.parse::<f64>().unwrap())),
            Expr::Operator(token_type, n) => Err(RuntimeError{message: format!("Received operator {:?} {} outside of expression", token_type, n)}),
            Expr::Unary(ref operator, ref rhs) => 
                match operator_from_expression(operator)? {
                    Operator::Bang => {
                        let result = self.visit_expr(rhs)?;
                        match result {
                            Object::Boolean(b) => Ok(Object::Boolean(!b)),
                            _ => Err(RuntimeError{message: format!("Bang operator received non-boolean expression: {:?}", result)}),
                        }
                    },
                    Operator::Subtract => {
                        let result = self.visit_expr(rhs)?;
                        match result {
                            Object::Float(float) => Ok(Object::Float(-float)),
                            Object::Integer(integer) => Ok(Object::Integer(-integer)),
                            _ => Err(RuntimeError{message: format!("Unary subtract operator received non-number expression: {:?}", result)}),
                        }
                    },
                    op => Err(RuntimeError{message: format!("Invalid unary opeartor {:?}", op)}),
                },
            Expr::Binary(ref lhs, ref operator, ref rhs) =>
                match operator_from_expression(operator)? {
                    Operator::BangEqual => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Boolean(lval != rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval != rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Boolean(lval as f64 != rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval != rval as f64)),
                            (Object::Boolean(lval), Object::Boolean(rval)) => Ok(Object::Boolean(lval != rval)),
                            (Object::StringLiteral(lval), Object::StringLiteral(rval)) => Ok(Object::Boolean(lval != rval)),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot compare using !=", lval, rval)}),
                        }
                    },
                    Operator::EqualEqual => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Boolean(lval == rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval == rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Boolean(lval as f64 == rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval == rval as f64)),
                            (Object::Boolean(lval), Object::Boolean(rval)) => Ok(Object::Boolean(lval == rval)),
                            (Object::StringLiteral(lval), Object::StringLiteral(rval)) => Ok(Object::Boolean(lval == rval)),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot compare using ==", lval, rval)}),
                        }
                    },
                    Operator::Greater => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Boolean(lval > rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval > rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Boolean(lval as f64 > rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval > rval as f64)),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot compare using >", lval, rval)}),
                        }
                    },
                    Operator::GreaterEqual => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Boolean(lval >= rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval >= rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Boolean(lval as f64 >= rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval >= rval as f64)),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot compare using >=", lval, rval)}),
                        }
                    },
                    Operator::Less => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Boolean(lval < rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval < rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Boolean((lval as f64) < rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval < (rval as f64))),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot compare using <", lval, rval)}),
                        }
                    },
                    Operator::LessEqual => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Boolean(lval <= rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval <= rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Boolean((lval as f64) <= rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Boolean(lval <= (rval as f64))),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot compare using <=", lval, rval)}),
                        }
                    },
                    Operator::Add => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Float(lval + rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Integer(lval + rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Float((lval as f64) + rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Float(lval + (rval as f64))),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot add", lval, rval)}),
                        }
                    },
                    Operator::Subtract => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Float(lval - rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Integer(lval - rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Float((lval as f64) - rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Float(lval - (rval as f64))),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot subtract", lval, rval)}),
                        }
                    },
                    Operator::Multiply => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Float(lval * rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Integer(lval * rval)),
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Float((lval as f64) * rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Float(lval * (rval as f64))),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot multiply", lval, rval)}),
                        }
                    },
                    Operator::Divide => {
                        let lhs_value = self.visit_expr(lhs)?;
                        let rhs_value = self.visit_expr(rhs)?;
                        match (lhs_value, rhs_value) {
                            (Object::Float(lval), Object::Float(rval)) => Ok(Object::Float(lval / rval)),
                            (Object::Integer(lval), Object::Integer(rval)) => Ok(Object::Float((lval as f64) / (rval as f64))), // DEFER: determine if this should be integer division
                            (Object::Integer(lval), Object::Float(rval)) => Ok(Object::Float((lval as f64) / rval)),
                            (Object::Float(lval), Object::Integer(rval)) => Ok(Object::Float(lval / (rval as f64))),
                            (lval, rval) => Err(RuntimeError{message: format!("lhs is {:?} rhs is {:?} cannot divide", lval, rval)}),
                        }
                    },
                    op => Err(RuntimeError{message: format!("Invalid inline opeartor {:?}", op)}),
                },
            Expr::Grouping(ref expr) => self.visit_expr(expr),
        }
    }

    fn visit_statement(&mut self, s: &Statement) -> Result<Object, RuntimeError> {
        match &*s {
            Statement::Expression(ref expr) => self.visit_expr(expr),
            Statement::Print(ref expr) => {
                let result = self.visit_expr(expr)?;
                println!("{}", stringify(&result));
                Ok(result)
            },
            Statement::Var(token, initializer) => {
                let value =
                    match initializer {
                        Some(ref expr) => self.visit_expr(expr)?,
                        None => Object::Nil()
                    };

                self.define_variable(token.lexeme.to_string(), value);
                Ok(Object::Nil())
            },
            Statement::Block(statements) => {
                self.execute_block(statements)?;
                Ok(Object::Nil())
            }
        }
    }
}

fn stringify(obj: &Object) -> String {
    match obj {
        Object::Nil() => format!("nil"),
        Object::Float(float) => format!("{}", float),
        Object::Integer(integer) => format!("{}", integer),
        Object::Boolean(boolean) => format!("{}", boolean),
        Object::StringLiteral(string) => format!("{}", string),
    }
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
