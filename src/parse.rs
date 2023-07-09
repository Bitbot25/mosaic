use crate::{
    span::Span,
    tokenize::{Token, TokenType, Tokenizer},
    SourceFile, SourceId,
};
use thiserror::Error;

#[derive(Debug)]
pub struct NodeBlock {
    name: Ident,
    parameters: Vec<BlockParameter>,
    ops: Vec<NodeOp>,
}

#[derive(Debug)]
pub struct BlockParameter {
    name: Ident,
    param_ty: Ident,
}

#[derive(Debug)]
pub struct Ident(String);

#[derive(Debug)]
pub struct NodeOp {}

struct TokenPeeker<'a> {
    peeked: Option<Token>,
    tokenizer: Tokenizer<'a>,
}

pub struct Parser<'a> {
    tokens: TokenPeeker<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a SourceFile) -> Self {
        Self {
            tokens: TokenPeeker::new(file),
        }
    }

    pub fn expect(&mut self, tt: TokenType) -> Result<Token, ParseError> {
        let tok = self.tokens.token();
        if tok.tok_type() == tt {
            Ok(tok)
        } else {
            Err(ParseError::UnexpectedToken {
                source_id: self.tokens.file().id(),
                expected_any: vec![tt],
                found: tok,
            })
        }
    }

    pub fn consume(&mut self, tt: TokenType) -> bool {
        let tok = self.tokens.token();
        tok.tok_type() == tt
    }

    pub fn sourced(&self, span: Span) -> &str {
        &self.tokens.file().text()[span.begin()..span.end()]
    }
    
    pub fn block_param(&mut self) -> Result<BlockParameter, ParseError> {

    }

    pub fn node_block(&mut self) -> Result<NodeBlock, ParseError> {
        self.expect(TokenType::KWBlock)?;
        let name_ident = self.expect(TokenType::Ident)?;
        let name = Ident(self.sourced(*name_ident.span()).to_string());
        self.expect(TokenType::LParen)?;

        let mut params = Vec::new();

        let mut tok = self.tokens.token();
        if tok.tok_type() != TokenType::RParen {
            loop {
                match tok.tok_type() {
                    TokenType::Ident => {
                        /* Parameter */
                        let name = Ident(self.sourced(*tok.span()).to_string());
                        self.expect(TokenType::Colon)?;
                        let param_ty_tok = self.expect(TokenType::Ident)?;
                        let param_ty = Ident(self.sourced(*param_ty_tok.span()).to_string());

                        let param = BlockParameter { name, param_ty };
                        params.push(param);

                        // TODO: Fix errors and add recovery

                        /* Comma or end of parameters */
                        let end = self.tokens.token();
                        match end.tok_type() {
                            TokenType::RParen => break,
                            TokenType::Comma => (),
                            _ => {
                                return Err(ParseError::UnexpectedToken {
                                    source_id: self.tokens.file().id(),
                                    found: end,
                                    expected_any: vec![TokenType::RParen, TokenType::Comma],
                                })
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            source_id: self.tokens.file().id(),
                            found: tok,
                            expected_any: vec![TokenType::RParen, TokenType::Ident],
                        })
                    }
                };
                tok = self.tokens.token();
            }
        }

        Ok(NodeBlock {
            name,
            parameters: params,
            ops: vec![],
        })
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unexpected token")]
    UnexpectedToken {
        source_id: SourceId,
        expected_any: Vec<TokenType>,
        found: Token,
    },
}
