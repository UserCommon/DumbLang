use token::{Token, TokenType};
#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expr: Box<Expr>
    },
    Literal {
        val: LiteralValue
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
}

impl Expr {
    pub fn accept<T: Visitor>(&self, &mut visitor: T) -> Result<T, Error> {
        match self {
            Expr::Binary {
                left,
                operator,
                right
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping {
                expr
            } => visitor.visit_grouping_expr(expr),
            Expr::Literal {
                val
            } => visitor.visit_literal_expr(val),
            Expr::Unary {
                operator,
                expr
            } => visitor.visit_unary_expr(operator, expr)
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: Token,
        right: &Expr
    ) -> Result<T, Error>;

    fn visit_grouping_expr(
        &mut self,
        expr: &Expr
    ) -> Result<T, Error>;

    fn visit_literal_expr(
        &mut self,
        value: &LiteralValue
    ) -> Result<T, Error>;

    fn visit_unary_expr(
        &mut self,
        expr: &Expr
    ) -> Result<T, Error>;
}

#[derive(Debug, Clone)]
enum Literal {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String)
}

// implements Visitor
struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> Result<String, Error>
    {
        expr.accept(self);
    }
}