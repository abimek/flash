use crate::lexer::*;
pub mod ast;
pub use ast::*;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
}

impl Parser {

    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            next_token: Token::Eof,
        };
        parser.bump();
        parser.bump();

        parser
    }

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn bump(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn current_token_is(&mut self, tok: Token) -> bool {
        self.current_token == tok
    }

    fn next_token_is(&mut self, tok: &Token) -> bool {
        self.next_token == *tok
    }

    fn expect_next_token(&mut self, tok: Token) -> bool {
        if self.next_token == tok {
            self.bump();
            return true;
        } else {
            panic!("Failre, expected token {:?}", tok);
        }
    }

    fn current_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.current_token)
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.next_token)
    }

    pub fn parse(&mut self) -> Vec<Program> {
        let mut program: Program = vec![];
        let mut program2: Program = vec![];
        while !self.current_token_is(Token::Eof) {
            match self.parse_stmt() {
                Some(mut stmt) => {
                    if let Stmt::Func{distributed: d, params: _, param_types: _, return_type: _,
                        body: _, name: _} = stmt {
                            if d {
                                program.push(stmt.clone());
                                program2.push(stmt)
                            }else{
                                program.push(stmt)
                            }
                    }else{

                        program.push(stmt)
                    }
                  /* if let Stmt::Expr(ref mut expr) = stmt {
                        match expr {
                            Expr::Func{distributed: d, params: _, param_types: _, return_type: _, 
                                body: _, name: _} => {
                                if *d {
                                    program.push(stmt.clone());
                                    program2.push(stmt)
                                }else{
                                    program.push(stmt)
                                }
                            },
                            _ => {
                                program.push(stmt)
                            }
                        }
                    }else{
                        program.push(stmt)
                    }*/
                },
                None => {}
            }
            self.bump()
        }
        return vec![program, program2];
    }

    pub fn parse_block_stmt(&mut self) -> BlockStmt {
        self.bump();

        let mut block = vec![];

        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {
            match self.parse_stmt() {
                Some(stmt) => block.push(stmt),
                None => {}
            }
            self.bump();
        }

        block
    }


    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current_token {
            Token::Let => self.parse_let_stmt(),
            Token::Return => self.parse_return_stmt(),
            Token::Ident(_) => self.parse_ident_stmt(), // Make sure to move this if we decide to do precidence
            Token::Dis => self.parse_dis_func_expr(),
            Token::Func => self.parse_func_expr(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_ident_stmt(&mut self) -> Option<Stmt> {
        if self.next_token_is(&Token::Assign) {
            return self.parse_assign_stmt();
        }
        return self.parse_expr_stmt();
    }


    fn parse_assign_stmt(&mut self) -> Option<Stmt> {
        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_next_token(Token::Assign) {
            return None;
        }

        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Assignment(name, expr))
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        match &self.next_token {
            Token::Ident(_) => self.bump(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        let value_type = self.parse_type().unwrap();

        if !self.expect_next_token(Token::Assign) {
            return None;
        }

        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Let(name, expr, value_type))
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Return(expr))
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&Token::Semicolon) {
                    self.bump();
                }
                Some(Stmt::Expr(expr))
            }
            None => None,
        }
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left = match self.current_token.clone() {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Int(_) => self.parse_int_expr(),
            // ADD STRING LATER
            Token::Bool(_) => self.parse_bool_expr(),
            Token::Minus | Token::Plus => self.parse_prefix_expr(),
            Token::LParen => self.parse_grouped_expr(),
            Token::If => self.parse_if_expr(),
       /*     Token::Dis => self.parse_dis_func_expr(),
            Token::Func => self.parse_func_expr(),*/
            d => {
                panic!("Failure, token: {:?}", d);
            }
        };

        while !self.next_token_is(&Token::Semicolon) && precedence < self.next_token_precedence() {
            match self.next_token {
                Token::Plus
                | Token::Minus
                | Token::Equal
                | Token::NotEqual
                | Token::LParen => {
                    self.bump();
                    left = self.parse_call_expr(left.unwrap());
                }
                _ => return left,
            }
        }
        left
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        match self.current_token {
            Token::Ident(ref mut ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        match self.parse_ident() {
            Some(ident) => Some(Expr::Ident(ident)),
            None => None,
        }
    }

    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Int(ref mut int) => Some(Expr::Literal(Literal::Int(int.clone()))),
            _ => None,
        }
    }

    fn parse_bool_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Bool(value) => Some(Expr::Literal(Literal::Bool(value == true))),
            _ => None,
        }
    }

    fn parse_expr_list(&mut self, end: Token) -> Option<Vec<Expr>> {
        let mut list = vec![];

        if self.next_token_is(&end) {
            self.bump();
            return Some(list);
        }

        self.bump();

        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => list.push(expr),
            None => return None,
        }

        while self.next_token_is(&Token::Comma) {
            self.bump();
            self.bump();

            match self.parse_expr(Precedence::Lowest) {
                Some(expr) => list.push(expr),
                None => return None,
            }
        }

        if !self.expect_next_token(end) {
            return None;
        }

        Some(list)
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.current_token {
            Token::Bang => Prefix::Not,
            Token::Minus => Prefix::Minus,
            Token::Plus => Prefix::Plus,
            _ => return None,
        };

        self.bump();

        match self.parse_expr(Precedence::Prefix) {
            Some(expr) => Some(Expr::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.bump();

        let expr = self.parse_expr(Precedence::Lowest);

        if !self.expect_next_token(Token::RParen) {
            None
        } else {
            expr
        }
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        if !self.expect_next_token(Token::LParen) {
            return None;
        } 

        self.bump();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::RParen) || !self.expect_next_token(Token::LBrace) {
            return None;
        }

        let consequence = self.parse_block_stmt();
        let mut alternative = None;

        if self.next_token_is(&Token::Else) {
            self.bump();

            if !self.expect_next_token(Token::LBrace) {
                return None;
            }

            alternative = Some(self.parse_block_stmt());
        }

        Some(Expr::If {
            cond: Box::new(cond),
            consequence,
            alternative,
        })
    }

    fn parse_dis_func_expr(&mut self) -> Option<Stmt> {
        if !self.expect_next_token(Token::Func) {
            return None;
        }

        match &self.next_token {
            Token::Ident(_) => self.bump(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_next_token(Token::LParen) {
            return None;
        }

        let (params, param_types) = match self.parse_func_params() {
            Some((params, param_types)) => (params, param_types),
            None => return None,
        };

        let return_type = self.parse_type().unwrap();

        if !self.expect_next_token(Token::LBrace) {
            return None;
        }

        Some(Stmt::Func {
            name: name.0,
            distributed: true,
            params: params,
            param_types: param_types,
            return_type: return_type,
            body: self.parse_block_stmt(),
        })

    }

    pub fn convert_token_to_expression_type(&mut self, token: Token) -> LLVMExpressionType {
        match token {
            Token::BoolType => LLVMExpressionType::Boolean,
            Token::IntType => LLVMExpressionType::Integer,
            _ => LLVMExpressionType::Null,
        }
    }

    fn parse_func_expr(&mut self) -> Option<Stmt> {
        match &self.next_token {
            Token::Ident(_) => self.bump(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_next_token(Token::LParen) {
            return None;
        }

        let (params, param_types) = match self.parse_func_params() {
            Some((params, param_types)) => (params, param_types),
            None => return None,
        };

        let return_type = self.parse_type().unwrap();

        if !self.expect_next_token(Token::LBrace) {
            return None;
        }

        Some(Stmt::Func {
            name: name.0,
            distributed: false,
            params: params,
            param_types: param_types,
            return_type: return_type,
            body: self.parse_block_stmt(),
        })
    }

    pub fn parse_type(&mut self) -> Option<LLVMExpressionType> {
        if !self.expect_next_token(Token::Colon) {
            return None;
        }

        self.bump();
        Some(self.convert_token_to_expression_type(self.current_token.clone()))
    }

    fn parse_func_params(&mut self) -> Option<(Vec<Ident>, Vec<LLVMExpressionType>)> {
        let mut params = vec![];
        let mut param_types = vec![];

        if self.next_token_is(&Token::RParen) {
            self.bump();
            return Some((params, param_types));
        }

        self.bump();

        match self.parse_ident() {
            Some(ident) => {
                params.push(ident);
                param_types.push(self.parse_type().unwrap())
            },
            None => return None,
        };

        while self.next_token_is(&Token::Comma) {
            self.bump();
            self.bump();

            match self.parse_ident() {
                Some(ident) => {
                    params.push(ident);
                    param_types.push(self.parse_type().unwrap())
                },
                None => return None,
            }
        }
        if !self.expect_next_token(Token::RParen) {
            return None;
        }

        Some((params, param_types))
    }

    fn parse_call_expr(&mut self, func: Expr) -> Option<Expr> {
        let args = match self.parse_expr_list(Token::RParen) {
            Some(args) => args,
            None => return None,
        };

        Some(Expr::Call {
            func: Box::new(func),
            args,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

  #[test]
    fn test_blank() {
        let input = String::from("
        1000;
        1000;
        1000;
        if (x) {
            x = 5;
        }");

        let mut parser = Parser::new(new_lexer(input));
        let programs = parser.parse();
        let program = programs.get(0).unwrap().clone();
        assert!(true);
        return;
        assert_eq!(
            vec![
                Stmt::Expr(Expr::Literal(Literal::Int(1000))),
                Stmt::Expr(Expr::Literal(Literal::Int(1000))),
                Stmt::Expr(Expr::Literal(Literal::Int(1000))),
                Stmt::Expr(Expr::If {
                    cond: Box::new(Expr::Ident(Ident(String::from("x")))),
                    consequence: vec![
                        Stmt::Assignment(Ident(String::from("x")), Expr::Literal(Literal::Int(1000)))
                    ],
                    alternative: None,
                }),
            ],
            program,
        );
    }

    #[test]
    fn test_func_ast_1() {
        let input = "func takevalues(x: int, y: int): int {takevalues()}";
        let mut parser = Parser::new(new_lexer(input));
        let programs = parser.parse();
        let program = programs.get(0).unwrap().clone();
        let program2 = programs.get(1).unwrap().clone();
        println!("{:?}", program);
        assert_eq!(
            vec![Stmt::Func{
                distributed: false,
                name: String::from("takevalues"),
                params: vec![Ident(String::from("x")), Ident(String::from("y"))],
                param_types: vec![LLVMExpressionType::Integer, LLVMExpressionType::Integer],
                return_type: LLVMExpressionType::Integer,
                body: vec![Stmt::Expr(Expr::Call{func: Box::new(Expr::Ident(Ident(String::from("takevalues")))), args: vec![]})],
            }],
            program,
        );
    }

    #[test]
    fn test_func_ast_2() {
        let input = "dis func takevalues(x: int, y: int): int {}";
        let mut parser = Parser::new(new_lexer(input));
        let programs = parser.parse();
        let program = programs.get(0).unwrap().clone();
        let program2 = programs.get(1).unwrap().clone();
        assert_eq!(
            vec![Stmt::Func{
                distributed: true,
                name: String::from("takevalues"),
                params: vec![Ident(String::from("x")), Ident(String::from("y"))],
                param_types: vec![LLVMExpressionType::Integer, LLVMExpressionType::Integer],
                return_type: LLVMExpressionType::Integer,
                body: vec![],
            }],
            program2,
        );
    }
}

