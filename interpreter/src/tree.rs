use crate::scanner::TokenType;

pub enum Expr {
    Unary {op: TokenType, expr: Box<Expr>},
    Literal(TokenType),
    Binary {left: Box<Expr>, right: Box<Expr>, op: TokenType},
    Grouping {expr: Box<Expr>}
}

macro_rules! paren {
    ($open:expr, $close:expr, $($id:expr),*) => {
        {
            let mut out = String::from($open);
            $(
                out.push_str(&$id.print());
            )*
            out.push($close);
            out
        }
    };
}

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Unary { op, expr } => paren!("(", ')', op, expr),
            Expr::Binary { left, right, op } => paren!("(", ')', left, right, op),
            Expr::Literal(value) => paren!("(", ')',value),
            Expr::Grouping { expr } => paren!("{", '}', expr),
        }
    }
}