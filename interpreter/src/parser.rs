use crate::{scanner::TokenType, tree::Expr, scanner::TokenType::*};

struct Parser<'a> {
    index: usize,
    tokens: &'a Vec<TokenType>
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<TokenType>) -> Self {
        Parser {
            index: 0,
            tokens
        }
    }
    fn error(token: TokenType, msg: String) {
        
    }
}

impl<'a> Parser<'a> {
    fn peek(&self) -> &TokenType {
        &self.tokens[self.index]
    }
    fn advance(&mut self) -> &TokenType {
        self.index += 1;
        &self.tokens[self.index-1]
    }
    fn prev(&self) -> &TokenType {
        &self.tokens[self.index-1]
    }
    fn check(&self, token: &TokenType) -> bool {
        if *token == TokenType::EOF {
            return false;
        }
        return match token {
            TokenType::Number(_) | TokenType::Identifier(_) | TokenType::TkString(_) => {
                std::mem::discriminant(token) == std::mem::discriminant(self.peek())
            }
            _ => self.peek() == token
        }
    }
    fn match_tk(&mut self, token_vec: Vec<TokenType>) -> bool {
        for token in token_vec {
            if(self.check(&token)) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn match_lit(&mut self) -> (TokenType, bool) {
        let peeked = self.peek().clone(); // optimize
        return match &peeked {
            TokenType::Number(_) | TokenType::TkString(_) => {
                (peeked, true)
            }
            _ => (TokenType::EOF, false)
        }
    }

    fn consume(&mut self, typ: TokenType, msg: String) -> &TokenType {
        if self.check(&typ) {
            return self.advance()
        }
        panic!("error: {}, {}", self.peek(), msg);
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparision();
        while self.match_tk(vec![BangEq, EqEq]) {
            let operator = self.prev().clone();
            let right = self.comparision();
            expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), op: operator }
        }
        expr
    }
    fn comparision(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_tk(vec![Gt, GtEq, Lt, LtEq]) {
            let operator = self.prev().clone();
            let right = self.term();
            expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), op: operator }
        }
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_tk(vec![Minus, Plus]) {
            let operator = self.prev().clone();
            let right = self.factor();
            expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), op: operator }
        }
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_tk(vec![Star, Slash]) {
            let operator = self.prev().clone();
            let right = self.unary();
            expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), op: operator }
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        if self.match_tk(vec![Bang, Minus]) {
            let operator = self.prev().clone();
            let right = self.unary();
            return Expr::Unary { op: operator, expr: Box::new(right) }
        }
        return self.primary()
    }

    fn primary(&mut self)  -> Expr {
        if self.match_tk(vec![TokenType::False]) {
            return Expr::Literal(TokenType::False)
        }
        if self.match_tk(vec![TokenType::True]) {
            return Expr::Literal(TokenType::True)
        }
        if self.match_tk(vec![TokenType::Nil]) {
            return Expr::Literal(TokenType::Nil)
        }
        let (token, mat) = self.match_lit();
        if mat {
            return Expr::Literal(token)
        }
        if self.match_tk(vec![LeftParan]) {
            self.consume(RightParan, "expectig )".to_string());
            return Expr::Grouping { expr: Box::new(Expr::Literal(TokenType::EOF)) }
        }
        panic!("gurt yo");
    }
}