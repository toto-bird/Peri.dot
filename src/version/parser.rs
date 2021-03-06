use std::collections::HashMap;

use super::exceptions::ParserException;
use super::tokens::*;
use super::nodes::*;
use super::lexer;



#[derive(Clone)]
pub struct ParseResult {
    pub exception: ParserException,
    pub node: Node,
    pub advancecount: usize
}
impl ParseResult {
    fn registeradvancement(&mut self) {
        self.advancecount += 1;
    }

    fn registerretreat(&mut self) {
        self.advancecount -= 1;
    }

    fn register(&mut self, res: ParseResult) -> Node {
        self.advancecount += res.advancecount;
        if res.exception.failed {
            self.exception = res.exception;
        }
        return res.node.clone();
    }

    fn success(&mut self, node: Node) -> ParseResult {
        self.node = node;
        return self.clone();
    }

    fn failure(&mut self, exception: ParserException) -> ParseResult {
        if ! self.exception.failed || self.advancecount == 0 {
            self.exception = exception;
        }
        return self.clone();
    }
}



pub enum ParseResponse {
    Success(Vec<Node>),
    Failed(ParserException)
}



#[derive(Clone)]
struct Parser {
    tokens: Vec<Token>,
    index: usize,
    curtoken: Token
}
impl Parser {
    fn advance(&mut self) -> Token {
        self.index += 1;
        if self.index < self.tokens.len() {
            self.curtoken = self.tokens[self.index].clone();
        }

        return self.curtoken.clone();
    }

    fn retreat(&mut self) -> Token {
        self.index -= 1;
        if self.index < self.tokens.len() {
            self.curtoken = self.tokens[self.index].clone();
        }

        return self.curtoken.clone();
    }



    fn parse(&mut self) -> ParseResponse {
        /*let mut res = self.expr();
        let tok = self.tokens[self.index + 1].clone();
        if ! res.exception.failed && tok.token != TT_EOF {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected Literal, EOF not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        return res;*/

        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let mut nodes = vec![];

        while self.curtoken.token != TT_EOF {
            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }

            if self.curtoken.token == TT_EOF {break}

            nodes.push(res.register(self.notraceexpr()));

            if res.exception.failed {
                return ParseResponse::Failed(res.exception);
            }
        }

        return ParseResponse::Success(nodes);
    }



    fn notraceexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};

        if self.curtoken.clone().matches(TT_KEYWORD, "var") {
            res.registeradvancement();
            self.advance();

            if self.curtoken.token != TT_IDENTIFIER {
                return res.failure(ParserException {
                    failed: true,
                    name: "SyntaxException".to_string(),
                    msg: "Expected IDENTIFIER not found".to_string(),
                    ucmsg: "Expected {} not found".to_string(),
                    start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                });
            }
            let varname = self.curtoken.clone();

            res.registeradvancement();
            self.advance();

            if self.curtoken.token != TT_EQUALS {
                return res.failure(ParserException {
                    failed: true,
                    name: "SyntaxException".to_string(),
                    msg: "Expected `=` not found".to_string(),
                    ucmsg: "Expected {} not found".to_string(),
                    start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                });
            }

            res.registeradvancement();
            self.advance();

            let expr = res.register(self.expr());
            if res.exception.failed {
                return res;
            }

            return res.success(Node {
                nodevalue: NodeValue::VarInitNode {
                    varname: varname.clone(), node: Box::new(expr)
                },
                start: varname.start.clone(), end: self.curtoken.start.clone()
            });


        } else if self.curtoken.token == TT_IDENTIFIER {
            let varname = self.curtoken.clone();

            res.registeradvancement();
            self.advance();

            if self.curtoken.token == TT_EQUALS {
                res.registeradvancement();
                self.advance();

                let expr = res.register(self.expr());

                if res.exception.failed {
                    return res;
                }

                return res.success(Node {
                    nodevalue: NodeValue::VarAssignNode {
                        varname: varname.clone(), node: Box::new(expr)
                    },
                    start: varname.start.clone(), end: self.curtoken.start.clone()
                });
            }

            res.registerretreat();
            self.retreat();
        }

        return self.expr();
    }



    fn expr(&mut self) -> ParseResult {
        return self.opexpr();
    }



    fn opexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let start = self.curtoken.start.clone();

        let mut value: ParseResult;
        let mut operations = vec![res.register(self.calltribute())];

        if res.exception.failed {
            return res;
        }

        loop {
            if self.curtoken.token == TT_NOT {
                let a = operations[operations.len() - 1].clone();
                operations.remove(operations.len() - 1);
                operations.push(Node {nodevalue: NodeValue::UnaryOpNode {node: Box::new(a), optoken: self.curtoken.clone()}, start: start.clone(), end: self.curtoken.end.clone()});
                res.registeradvancement();
                self.advance();
                continue;

            } else if self.curtoken.token == TT_LSSTHN {
                res.registeradvancement();
                self.advance();

                if self.curtoken.token == TT_TYPE {
                    let typeexpr = res.register(self.typeexpr());
    
                    if res.exception.failed {
                        return res;
                    }
    
                    let casttype = match typeexpr.nodevalue {
                        NodeValue::TypeNode {value} => value,
                        _ => panic!("Non TypeNode received")
                    };

                    if self.curtoken.token != TT_GRTTHN {
                        return res.failure(ParserException {
                            failed: true,
                            name: "SyntaxException".to_string(),
                            msg: "Expected `>` not found".to_string(),
                            ucmsg: "Expected {} not found".to_string(),
                            start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                        });
                    }

                    let a = operations[operations.len() - 1].clone();
                    operations.remove(operations.len() - 1);
                    operations.push(Node {nodevalue: NodeValue::CastOpNode {casttype: casttype, node: Box::new(a)}, start: start.clone(), end: self.curtoken.end.clone()});

                    res.registeradvancement();
                    self.advance();

                } else {
                    res.registerretreat();
                    self.retreat();
                }
            }

            let bktoken = self.curtoken.clone();
            let bkindex = self.index.clone();
            let bkres   = res.clone();

            value = self.calltribute();

            if ! value.exception.failed {
                operations.push(value.node);
                continue;
            }

            self.curtoken = bktoken;
            self.index = bkindex;
            res = bkres;

            if [
                TT_PLUS, TT_MINUS, TT_TIMES, TT_DIVBY, TT_POW,
                TT_EQUALEQ, TT_NOTEQ, TT_LSSTHN, TT_GRTTHN, TT_LSSTHNEQ, TT_GRTTHNEQ,
                TT_AND, TT_XOR, TT_OR
            ].contains(&self.curtoken.token.as_str()) {
                if operations.len() < 2 {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected Literal, `(`, `-` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }
                let a = operations[operations.len() - 2].clone();
                let b = operations[operations.len() - 1].clone();
                operations.remove(operations.len() - 1);
                operations.remove(operations.len() - 1);
                operations.push(Node {nodevalue: NodeValue::BinaryOpNode {left: Box::new(a), optoken: self.curtoken.clone(), right: Box::new(b)}, start: start.clone(), end: self.curtoken.end.clone()});

                res.registeradvancement();
                self.advance();

            } else {
                break
            }
        }

        if operations.len() > 1 {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected Operator, Comparison, Combiner not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        return res.success(operations[0].clone());
    }



    fn calltribute(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let start = self.curtoken.start.clone();

        let mut factor = res.register(self.factor());

        if res.exception.failed {
            return res;
        }

        loop {
            if self.curtoken.token == TT_LPAREN {
                res.registeradvancement();
                self.advance();

                while self.curtoken.token == TT_EOL {
                    res.registeradvancement();
                    self.advance();
                }

                let mut args = vec![];

                let end: lexer::LexerPosition;

                if self.curtoken.token == TT_RPAREN {
                    end = self.curtoken.end.clone();

                    res.registeradvancement();
                    self.advance();

                } else {
                    args.push(res.register(self.expr()));

                    if res.exception.failed {
                        return res;
                    }

                    while self.curtoken.token == TT_COMMA {
                        res.registeradvancement();
                        self.advance();

                        while self.curtoken.token == TT_EOL {
                            res.registeradvancement();
                            self.advance();
                        }

                        args.push(res.register(self.expr()));

                        if res.exception.failed {
                            return res;
                        }
                    }

                    while self.curtoken.token == TT_EOL {
                        res.registeradvancement();
                        self.advance();
                    }

                    if self.curtoken.token != TT_RPAREN {
                        return res.failure(ParserException {
                            failed: true,
                            name: "SyntaxException".to_string(),
                            msg: "Expected `)` not found".to_string(),
                            ucmsg: "Expected {} not found".to_string(),
                            start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                        });
                    }

                    end = self.curtoken.end.clone();

                    res.registeradvancement();
                    self.advance();
                }

                factor = Node {
                    nodevalue: NodeValue::CallNode {
                        varname: Box::new(factor),
                        args: args
                    },
                    start: start.clone(), end: end
                };


            } else if self.curtoken.token == TT_DOT {
                res.registeradvancement();
                self.advance();

                if self.curtoken.token != TT_IDENTIFIER {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected identifier not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                factor = Node {
                    nodevalue: NodeValue::AttributeNode {
                        varname: Box::new(factor),
                        attribute: self.curtoken.clone()
                    },
                    start: start.clone(), end: self.curtoken.end.clone()
                };

                res.registeradvancement();
                self.advance();


            } else {
                break;
            }
        }

        return res.success(factor);
    }



    fn factor(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let token = self.curtoken.clone();

        if [TT_MINUS].contains(&token.token.as_str()) {
            let start = self.curtoken.start.clone();
            res.registeradvancement();
            self.advance();

            let factor = res.register(self.factor());
            if res.exception.failed {
                return res;
            }

            return res.success(Node {
                nodevalue: NodeValue::UnaryOpNode {
                    optoken: token,
                    node: Box::new(factor.clone())
                },
                start: start, end: factor.end
            });


        } else if token.clone().matches(TT_KEYWORD, "if") {
            let ifexpr = res.register(self.ifexpr());
            if res.exception.failed {
                return res;
            }
            return res.success(ifexpr);


        } else if token.clone().matches(TT_KEYWORD, "for") {
            let forexpr = res.register(self.forexpr());
            if res.exception.failed {
                return res;
            }
            return res.success(forexpr);


        } else if token.clone().matches(TT_KEYWORD, "while") {
            let whileexpr = res.register(self.whileexpr());
            if res.exception.failed {
                return res;
            }
            return res.success(whileexpr);


        } else if token.clone().matches(TT_KEYWORD, "func") {
            let funcexpr = res.register(self.funcexpr());
            if res.exception.failed {
                return res;
            }
            return res.success(funcexpr);


        } else {
            let atom = res.register(self.atom());

            if res.exception.failed {
                return res;
            }

            return res.success(atom);
        }
    }



    fn ifexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let start = self.curtoken.start.clone();
        let mut cases = vec![];
        let mut elsecase = None;

        if ! self.curtoken.clone().matches(TT_KEYWORD, "if") {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `if` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        if self.curtoken.token != TT_LPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `(` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        let condition = res.register(self.expr());
        if res.exception.failed {
            return res;
        }

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        if self.curtoken.token != TT_RPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `)` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        let response = self.codeblock();
        let codeblock: Vec<Node>;
        match response {
            ParseResponse::Success(value) => {
                codeblock = value;
            },
            ParseResponse::Failed(err) => {return res.failure(err)}
        }
        cases.push((condition, codeblock));

        let mut i = 0;
        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
            i += 1;
        }


        if ! self.curtoken.clone().matches(TT_KEYWORD, "elif") {
            for _ in 0 .. i {
                res.registerretreat();
                self.retreat();
            }
        }
        
        while self.curtoken.clone().matches(TT_KEYWORD, "elif") {
            res.registeradvancement();
            self.advance();

            if self.curtoken.token != TT_LPAREN {
                return res.failure(ParserException {
                    failed: true,
                    name: "SyntaxException".to_string(),
                    msg: "Expected `(` not found".to_string(),
                    ucmsg: "Expected {} not found".to_string(),
                    start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                });
            }

            res.registeradvancement();
            self.advance();

            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }

            let condition = res.register(self.expr());
            if res.exception.failed {
                return res;
            }

            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }

            if self.curtoken.token != TT_RPAREN {
                return res.failure(ParserException {
                    failed: true,
                    name: "SyntaxException".to_string(),
                    msg: "Expected `)` not found".to_string(),
                    ucmsg: "Expected {} not found".to_string(),
                    start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                });
            }

            res.registeradvancement();
            self.advance();

            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }

            let response = self.codeblock();
            let codeblock: Vec<Node>;
            match response {
                ParseResponse::Success(value) => {
                    codeblock = value;
                },
                ParseResponse::Failed(err) => {return res.failure(err)}
            }
            cases.push((condition, codeblock));

            let mut i = 0;
            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
                i += 1;
            }

            if ! self.curtoken.clone().matches(TT_KEYWORD, "elif") {
                for _ in 0 .. i {
                    res.registerretreat();
                    self.retreat();
                }
            }
        }

        let mut i = 0;
        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
            i += 1;
        }


        if ! self.curtoken.clone().matches(TT_KEYWORD, "else") {
            for _ in 0 .. i {
                res.registerretreat();
                self.retreat();
            }
        }


        if self.curtoken.clone().matches (TT_KEYWORD, "else") {
            res.registeradvancement();
            self.advance();

            let response = self.codeblock();
            let codeblock: Vec<Node>;
            match response {
                ParseResponse::Success(value) => {
                    codeblock = value;
                },
                ParseResponse::Failed(err) => {return res.failure(err)}
            }
            elsecase = Some(codeblock);
        }

        return res.success(Node {
            nodevalue: NodeValue::IfNode {
                cases, elsecase
            },
            start, end: self.curtoken.end.clone()
        });
    }



    fn forexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let start = self.curtoken.start.clone();

        if ! self.curtoken.clone().matches(TT_KEYWORD, "for") {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `for` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        let mut varoverwrite = false;
        if self.curtoken.clone().matches(TT_KEYWORD, "var") {
            varoverwrite = true;
            res.registeradvancement();
            self.advance();
        }

        if self.curtoken.token != TT_IDENTIFIER {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected IDENTIFIER not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        let varname = self.curtoken.clone();

        res.registeradvancement();
        self.advance();

        if ! self.curtoken.clone().matches(TT_KEYWORD, "in") {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `in` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        if self.curtoken.token != TT_LPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `(` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        let expr = res.register(self.expr());

        if res.exception.failed {
            return res;
        }

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        if self.curtoken.token != TT_RPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `)` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        let response = self.codeblock();
        let codeblock: Vec<Node>;
        match response {
            ParseResponse::Success(value) => {
                codeblock = value;
            },
            ParseResponse::Failed(err) => {return res.failure(err)}
        }

        return res.success(Node {
            nodevalue: NodeValue::ForNode {
                varoverwrite: varoverwrite,
                varname: varname,
                iterable: Box::new(expr),
                body: codeblock
            },
            start: start, end: self.curtoken.start.clone()
        });
    }



    fn whileexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let start = self.curtoken.start.clone();

        if ! self.curtoken.clone().matches(TT_KEYWORD, "while") {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `while` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        if self.curtoken.token != TT_LPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `(` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        let condition = res.register(self.expr());

        if res.exception.failed {
            return res;
        }

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        if self.curtoken.token != TT_RPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `)` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        let response = self.codeblock();
        let codeblock: Vec<Node>;
        match response {
            ParseResponse::Success(value) => {
                codeblock = value;
            },
            ParseResponse::Failed(err) => {return res.failure(err)}
        }

        return res.success(Node {
            nodevalue: NodeValue::WhileNode {
                condition: Box::new(condition),
                body: codeblock
            },
            start: start, end: self.curtoken.start.clone()
        });
    }



    fn funcexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let start = self.curtoken.start.clone();

        if ! self.curtoken.clone().matches(TT_KEYWORD, "func") {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `func` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        if self.curtoken.token != TT_LPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `(` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        let mut args: HashMap<i32, (Token, String)> = HashMap::new();

        if self.curtoken.token == TT_IDENTIFIER {
            let nametoken = self.curtoken.clone();

            res.registeradvancement();
            self.advance();

            if self.curtoken.token != TT_COLON {
                return res.failure(ParserException {
                    failed: true,
                    name: "SyntaxException".to_string(),
                    msg: "Expected `:` not found".to_string(),
                    ucmsg: "Expected {} not found".to_string(),
                    start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                });
            }

            res.registeradvancement();
            self.advance();

            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }

            let typeexpr = res.register(self.typeexpr());

            if res.exception.failed {
                return res;
            }

            let mut i = 0;

            args.insert(i, (nametoken, match typeexpr.nodevalue {
                NodeValue::TypeNode {value} => value,
                _ => panic!("Non TypeNode received")
            }));

            while self.curtoken.token == TT_COMMA {
                res.registeradvancement();
                self.advance();

                i += 1;

                while self.curtoken.token == TT_EOL {
                    res.registeradvancement();
                    self.advance();
                }

                if self.curtoken.token != TT_IDENTIFIER {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected IDENTIFIER not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                let nametoken = self.curtoken.clone();
    
                res.registeradvancement();
                self.advance();
    
                if self.curtoken.token != TT_COLON {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `:` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }
    
                res.registeradvancement();
                self.advance();

                while self.curtoken.token == TT_EOL {
                    res.registeradvancement();
                    self.advance();
                }

                let typeexpr = res.register(self.typeexpr());
    
                if res.exception.failed {
                    return res;
                }
    
                args.insert(i, (nametoken, match typeexpr.nodevalue {
                    NodeValue::TypeNode {value} => value,
                    _ => panic!("Non TypeNode received")
                }));
            }
        }

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        if self.curtoken.token != TT_RPAREN {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `(` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();
    
        if self.curtoken.token != TT_COLON {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `:` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        let returntype = res.register(self.typeexpr());

        if res.exception.failed {
            return res;
        }

        let response = self.codeblock();
        let codeblock: Vec<Node>;
        match response {
            ParseResponse::Success(value) => {
                codeblock = value;
            },
            ParseResponse::Failed(err) => {return res.failure(err)}
        }

        return res.success(Node {
            nodevalue: NodeValue::FuncNode {
                args: args,
                returntype: Box::new(returntype),
                body: codeblock
            },
            start: start, end: self.curtoken.end.clone()
        });
    }



    fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let token = self.curtoken.clone();
        let value = token.value.clone();
        let start = token.start.clone();
        let end = token.end.clone();

        if token.token == TT_INT {
            res.registeradvancement();
            self.advance();
            return res.success(Node {
                nodevalue: NodeValue::IntNode {
                    token, value
                },
                start, end: end
            });

        } else if token.token == TT_FLOAT {
            res.registeradvancement();
            self.advance();
            return res.success(Node {
                nodevalue: NodeValue::FloatNode {
                    token, value
                },
                start, end: end
            });

        } else if token.token == TT_STRING {
            res.registeradvancement();
            self.advance();
            return res.success(Node {
                nodevalue: NodeValue::StringNode {
                    token, value
                },
                start, end: end
            });

        } else if token.token == TT_IDENTIFIER {
            res.registeradvancement();
            self.advance();
            return res.success(Node {
                nodevalue: NodeValue::VarAccessNode {
                    token
                },
                start, end: end
            });

        } else if token.token == TT_LSQUARE {
            return self.arrayexpr();

        } else {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected Literal, Identifier not found.".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }
    }



    fn arrayexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let start = self.curtoken.start.clone();

        if self.curtoken.token != TT_LSQUARE {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `[` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        if self.curtoken.token == TT_COMMA {
            res.registeradvancement();
            self.advance();

            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }

            if self.curtoken.token != TT_RSQUARE {
                return res.failure(ParserException {
                    failed: true,
                    name: "SyntaxException".to_string(),
                    msg: "Expected `]` not found".to_string(),
                    ucmsg: "Expected {} not found".to_string(),
                    start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                });
            }

            res.registeradvancement();
            self.advance();

            return res.success(Node {
                nodevalue: NodeValue::ArrayNode {
                    exprs: vec![]
                },
                start: start, end: self.curtoken.end.clone()
            });
        }

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        if self.curtoken.token == TT_RSQUARE {
            let end = self.curtoken.end.clone();

            res.registeradvancement();
            self.advance();

            return res.success(Node {
                nodevalue: NodeValue::ArrayNode {
                    exprs: vec![]
                },
                start: start, end: end
            });
        }

        let mut exprs = vec![res.register(self.expr())];

        if res.exception.failed {
            return res;
        }

        while self.curtoken.token == TT_COMMA {
            res.registeradvancement();
            self.advance();

            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }

            if self.curtoken.token == TT_RSQUARE {
                break
            }

            exprs.push(res.register(self.expr()));

            if res.exception.failed {
                return res;
            }
        }

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        if self.curtoken.token != TT_RSQUARE {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `]` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        let end = self.curtoken.end.clone();

        res.registeradvancement();
        self.advance();

        return res.success(Node {
            nodevalue: NodeValue::ArrayNode {
                exprs: exprs
            },
            start: start, end: end
        })
    }



    fn typeexpr(&mut self) -> ParseResult {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};
        let token = self.curtoken.clone();

        if token.token == TT_TYPE {
            if token.value == "Str" {
                res.registeradvancement();
                self.advance();

                return res.success(Node {
                    nodevalue: NodeValue::TypeNode {
                        value: "Str".to_string()
                    },
                    start: token.start.clone(), end: token.end.clone()
                });

            } else if token.value == "Int" {
                res.registeradvancement();
                self.advance();

                return res.success(Node {
                    nodevalue: NodeValue::TypeNode {
                        value: "Int".to_string()
                    },
                    start: token.start.clone(), end: token.end.clone()
                });

            } else if token.value == "Float" {
                res.registeradvancement();
                self.advance();

                return res.success(Node {
                    nodevalue: NodeValue::TypeNode {
                        value: "Float".to_string()
                    },
                    start: token.start.clone(), end: token.end.clone()
                });

            } else if token.value == "Bool" {
                res.registeradvancement();
                self.advance();

                return res.success(Node {
                    nodevalue: NodeValue::TypeNode {
                        value: "Bool".to_string()
                    },
                    start: token.start.clone(), end: token.end.clone()
                });

            } else if token.value == "Array" {
                let start = token.start.clone();

                res.registeradvancement();
                self.advance();

                if self.curtoken.token != TT_LSSTHN {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `<` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                res.registeradvancement();
                self.advance();

                if self.curtoken.token != TT_INT {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected INT not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                let len = self.curtoken.value.clone();

                res.registeradvancement();
                self.advance();

                if self.curtoken.token != TT_COMMA {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `,` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                res.registeradvancement();
                self.advance();

                let typ = res.register(self.typeexpr());

                if res.exception.failed {
                    return res;
                }

                let typ = match typ.nodevalue {
                    NodeValue::TypeNode {value} => value,
                    _ => panic!("Typeexpr returned non TypeNode")
                };

                if self.curtoken.token != TT_GRTTHN {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `>` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }
                
                let end = self.curtoken.end.clone();

                res.registeradvancement();
                self.advance();

                return res.success(Node {
                    nodevalue: NodeValue::TypeNode {
                        value: format!("Array<{}, {}>", len, typ)
                    },
                    start: start, end: end
                });

            } else if token.value == "Func" {
                let start = token.start.clone();

                res.registeradvancement();
                self.advance();

                if self.curtoken.token != TT_LSSTHN {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `<` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                res.registeradvancement();
                self.advance();

                if self.curtoken.token != TT_LSQUARE {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `[` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                res.registeradvancement();
                self.advance();

                let mut params = vec![];

                if self.curtoken.token == TT_RSQUARE {
                    res.registeradvancement();
                    self.advance();

                } else {

                    let typ = res.register(self.typeexpr());

                    if res.exception.failed {
                        return res;
                    }

                    params.push(match typ.nodevalue {
                        NodeValue::TypeNode {value} => value,
                        _ => panic!("Typeexpr returned non TypeNode")
                    });

                    while self.curtoken.token == TT_COMMA {
                        res.registeradvancement();
                        self.advance();

                        let typ = res.register(self.typeexpr());

                        if res.exception.failed {
                            return res;
                        }

                        params.push(match typ.nodevalue {
                            NodeValue::TypeNode {value} => value,
                            _ => panic!("Typeexpr returned non TypeNode")
                        });
                    }

                    if self.curtoken.token != TT_RSQUARE {
                        return res.failure(ParserException {
                            failed: true,
                            name: "SyntaxException".to_string(),
                            msg: "Expected `]` not found".to_string(),
                            ucmsg: "Expected {} not found".to_string(),
                            start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                        });
                    }

                    res.registeradvancement();
                    self.advance();
                }

                if self.curtoken.token != TT_COMMA {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `,` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }

                res.registeradvancement();
                self.advance();

                let typ = res.register(self.typeexpr());

                if res.exception.failed {
                    return res;
                }

                let returntype = match typ.nodevalue {
                    NodeValue::TypeNode {value} => value,
                    _ => panic!("Typeexpr returned non TypeNode")
                };

                if self.curtoken.token != TT_GRTTHN {
                    return res.failure(ParserException {
                        failed: true,
                        name: "SyntaxException".to_string(),
                        msg: "Expected `>` not found".to_string(),
                        ucmsg: "Expected {} not found".to_string(),
                        start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
                    });
                }
                
                let end = self.curtoken.end.clone();

                res.registeradvancement();
                self.advance();

                let mut paramstr = "".to_string();

                for i in 0 .. params.len() {
                    paramstr += params[i].as_str();

                    if i < params.len() - 1 {
                        paramstr += ", "
                    }
                }

                return res.success(Node {
                    nodevalue: NodeValue::TypeNode {
                        value: format!("Func<[{}], {}>", paramstr, returntype)
                    },
                    start: start, end: end
                });

            } else {
                panic!("Unknown type value found");
            }

        } else {
            return res.failure(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected Type not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: token.start.clone(), end: token.end.clone()
            });
        }
    }



    fn codeblock(&mut self) -> ParseResponse {
        let mut res = ParseResult {exception: ParserException {failed: false, name: "".to_string(), msg: "".to_string(), ucmsg: "".to_string(), start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, node: Node {nodevalue: NodeValue::NullNode, start: self.curtoken.start.clone(), end: self.curtoken.end.clone()}, advancecount: 0};

        if self.curtoken.token != TT_LCURLY {
            return ParseResponse::Failed(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `{` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        while self.curtoken.token == TT_EOL {
            res.registeradvancement();
            self.advance();
        }

        let mut exprs = vec![];
        while self.curtoken.token != TT_EOF && self.curtoken.token != TT_RCURLY {
            exprs.push(res.register(self.notraceexpr()));
            if res.exception.failed {
                return ParseResponse::Failed(res.exception)
            }

            while self.curtoken.token == TT_EOL {
                res.registeradvancement();
                self.advance();
            }
        }

        if self.curtoken.token != TT_RCURLY {
            return ParseResponse::Failed(ParserException {
                failed: true,
                name: "SyntaxException".to_string(),
                msg: "Expected `}` not found".to_string(),
                ucmsg: "Expected {} not found".to_string(),
                start: self.curtoken.start.clone(), end: self.curtoken.end.clone()
            });
        }

        res.registeradvancement();
        self.advance();

        return ParseResponse::Success(exprs);
    }
}



pub fn parse(tokens: Vec<Token>) -> ParseResponse {
    let curtoken = tokens[0].clone();
    let mut parser = Parser {
        tokens: tokens,
        index: 0,
        curtoken: curtoken
    };

    let result = parser.parse();
    return result;
}
