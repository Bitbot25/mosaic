use std::str::Chars;

use crate::{span::Span, SourceFile};
use thiserror::Error;
use unicode_xid::UnicodeXID;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum TokenType {
    Equal,
    Ident,
    KWBlock,
    LParen,
    RParen,
    Colon,
    Comma,
    Open,
    Close,
    LAngle,
    RAngle,

    Eof,
    Invalid,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Equal => write!(f, "'='"),
            TokenType::Ident => write!(f, "<identifier>"),
            TokenType::KWBlock => write!(f, "'block'"),
            TokenType::LParen => write!(f, "'('"),
            TokenType::RParen => write!(f, "')'"),
            TokenType::Colon => write!(f, "':'"),
            TokenType::Comma => write!(f, "','"),
            TokenType::Open => write!(f, "'{{'"),
            TokenType::Close => write!(f, "'}}'"),
            TokenType::LAngle => write!(f, "'<'"),
            TokenType::RAngle => write!(f, "'>'"),
            TokenType::Eof => write!(f, "<eof>"),
            TokenType::Invalid => write!(f, "<invalid>"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    tt: TokenType,
    span: Span,
}

impl Token {
    pub fn tok_type(&self) -> TokenType {
        self.tt
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    offset: usize,
    file: &'a SourceFile,
}

impl<'a> Tokenizer<'a> {
    pub fn new(file: &'a SourceFile) -> Self {
        Self {
            chars: file.text.chars(),
            offset: 0,
            file,
        }
    }

    pub fn file(&self) -> &SourceFile {
        &self.file
    }

    pub fn will_eof(&self) -> bool {
        for c in self.chars.clone() {
            if !is_whitespace(c) {
                return false;
            }
        }
        true
    }

    pub fn is_eof(&self) -> bool {
        self.peek0().is_none()
    }

    pub fn peek0(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(_) = c {
            self.offset += 1;
        }
        c
    }

    pub fn eat_while(&mut self, mut will_eat: impl FnMut(char) -> bool) {
        while match self.peek0() {
            Some(next) => will_eat(next),
            None => false,
        } {
            self.advance();
        }
    }

    pub fn token(&mut self) -> Token {
        self.eat_while(is_whitespace);

        let begin = self.offset;
        let init = match self.advance() {
            Some(c) => c,
            None => {
                return Token {
                    tt: TokenType::Eof,
                    span: Span::point(self.offset),
                }
            }
        };

        let tt = if UnicodeXID::is_xid_start(init) {
            self.eat_while(UnicodeXID::is_xid_continue);
            let slice = &self.file.text[begin..self.offset];

            match slice {
                "block" => TokenType::KWBlock,
                _ => TokenType::Ident,
            }
        } else {
            match init {
                '=' => TokenType::Equal,
                '(' => TokenType::LParen,
                ')' => TokenType::RParen,
                ':' => TokenType::Colon,
                ',' => TokenType::Comma,
                '{' => TokenType::Open,
                '}' => TokenType::Close,
                '<' => TokenType::LAngle,
                '>' => TokenType::RAngle,
                _ => TokenType::Invalid,
            }
        };

        Token {
            tt,
            span: Span::new(begin..self.offset),
        }
    }
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}
