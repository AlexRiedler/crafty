use crate::parser::Visitor;
use crate::parser::Expr;
use crate::parser::Statement;

pub struct AstPrinter {
    pub indent: u32,
}
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
            Expr::Logical(ref lhs, token_type, ref rhs) => format!("{} {:?} {}", self.visit_expr(lhs), token_type, self.visit_expr(rhs)),
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
            Statement::If(ref expr, ref then_statement, ref else_branch) => match else_branch {
                Some(else_statement) => format!("if {} then {} else {}", self.visit_expr(expr), self.visit_statement(then_statement), self.visit_statement(else_statement)),
                None => format!("if {} then {}", self.visit_expr(expr), self.visit_statement(then_statement)),
            },
            Statement::Print(ref expr) => format!("print {};", self.visit_expr(expr)),
            Statement::While(ref condition, ref body) => format!("while {} {}", self.visit_expr(condition), self.visit_statement(body)),
            Statement::Var(token, initializer) => {
                match initializer {
                    Some(expr) => format!("var {} = {};", token.lexeme.to_string(), self.visit_expr(expr)),
                    None => format!("var {};", token.lexeme.to_string()),
                }
            },
            Statement::Block(statements) => {
                let mut s = String::new();
                s.push('{');
                s.push('\n');

                self.indent += 2;
                let string = statements.iter()
                    .map(|statement| left_pad(self.indent, self.visit_statement(statement)))
                    .collect::<Vec<String>>()
                    .join("\n");
                s.push_str(&string);
                s.push('\n');
                self.indent -= 2;

                s.push_str(&left_pad(self.indent, "}".to_string()));
                s
            }
        }
    }
}

// TODO: use trait?
fn left_pad(amount: u32, string: String) -> String {
    let mut s = String::new();
    for _ in 0..amount { s.push(' ') }
    s.push_str(&string);
    s
}

