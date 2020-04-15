use crate::parser::Visitor;
use crate::parser::Expr;
use crate::parser::Statement;

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&mut self, statements: &Vec<Statement>) {
        for statement in statements.iter() {
            let string = self.visit_statement(statement);
            println!("{}", string);
        }
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        match &*e {
            Expr::BoolLiteral(b) => format!("{}", b),
            Expr::StringLiteral(n) => n.to_string(),
            Expr::IntegerLiteral(n) => n.to_string(),
            Expr::FloatLiteral(n) => n.to_string(),
            Expr::Operator(_token_type, n) => n.to_string(),
            Expr::Unary(ref operator, ref rhs) => format!("({} {})", self.visit_expr(operator), self.visit_expr(rhs)),
            Expr::Binary(ref lhs, ref operator, ref rhs) => format!("({} {} {})", self.visit_expr(operator), self.visit_expr(lhs), self.visit_expr(rhs)),
            Expr::Grouping(ref expr) => format!("{}", self.visit_expr(expr)),
            Expr::Variable(token) => format!("{}", token.lexeme.to_string()),
            Expr::Assign(token, ref expr) => format!("{} = {}", token.lexeme.to_string(), self.visit_expr(expr)),
        }
    }

    fn visit_statement(&mut self, s: &Statement) -> String {
        match &*s {
            Statement::Expression(ref expr) => self.visit_expr(expr),
            Statement::Print(ref expr) => format!("print {}", self.visit_expr(expr)),
            Statement::Var(token, initializer) => {
                match initializer {
                    Some(expr) => format!("var {} = {}", token.lexeme.to_string(), self.visit_expr(expr)),
                    None => format!("var {};", token.lexeme.to_string()),
                }
            }
        }
    }
}

