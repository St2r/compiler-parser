use std::env;
use std::path::Iter;
use std::fmt::{Display, Formatter, Error, Result};
use std::str::Chars;
use std::borrow::Borrow;
use crate::TokenType::Unknown;

fn main() {
    let mut c = Content { content: "a1 a1 a1" };
    for i in c {
        println!("{}", i);
        match i.token_type {
            TokenType::Unknown => { break; }
            _ => {}
        }
    }
}

enum TokenType {
    Identify,
    Keyword,
    UnsignedNumber,
    Delimiter,
    Unknown,
}

struct Content<'a> {
    content: &'a str,
}

impl<'a> IntoIterator for Content<'a> {
    type Item = Token;
    type IntoIter = TokenIterator;

    fn into_iter(self) -> Self::IntoIter {
        TokenIterator {
            chars: self.content.chars().collect(),
            cur_index: 0,
            token: "".to_string(),
        }
    }
}

struct TokenIterator {
    chars: Vec<char>,
    cur_index: usize,
    token: String,
}

struct Token {
    token_type: TokenType,
    content: String,
}

impl Token {
    fn keyword(value: &str) -> Token {
        return Token {
            token_type: TokenType::Keyword,
            content: value.to_string(),
        };
    }

    fn identify(value: &str) -> Token {
        Token {
            token_type: TokenType::Identify,
            content: value.to_string(),
        }
    }

    fn unsigned_number(value: &str) -> Token {
        Token {
            token_type: TokenType::UnsignedNumber,
            content: value.to_string(),
        }
    }

    fn delimiter(value: &str) -> Token {
        Token {
            token_type: TokenType::Delimiter,
            content: value.to_string(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.token_type {
            TokenType::Identify => {
                write!(f, "Ident({})", self.content)
            }
            TokenType::Keyword => {
                write!(f, "{}", self.content)
            }
            TokenType::UnsignedNumber => {
                write!(f, "Int{}", self.content)
            }
            TokenType::Delimiter => {
                write!(f, "{}", self.content)
            }
            TokenType::Unknown => {
                write!(f, "Unknown")
            }
        }
    }
}

impl<'a> Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.token.clear();

        let mut ch = ' ';
        while ch.is_whitespace() {
            if self.cur_index == self.chars.len() {
                return None;
            } else {
                ch = self.chars[self.cur_index];
                self.cur_index += 1;
            }
        }
        return if ch.is_alphabetic() {
            while ch.is_alphanumeric() {
                self.token.push(ch);
                if self.cur_index == self.chars.len() {
                    break;
                } else {
                    ch = self.chars[self.cur_index];
                    self.cur_index += 1;
                }
            }
            if self.cur_index != self.chars.len() {
                self.cur_index -= 1;
            }

            match self.token.as_str() {
                "BEGIN" => Some(Token::keyword("Begin")),
                "END" => Some(Token::keyword("End")),
                "FOR" => Some(Token::keyword("For")),
                "IF" => Some(Token::keyword("If")),
                "THEN" => Some(Token::keyword("Then")),
                "ELSE" => Some(Token::keyword("Else")),
                _ => Some(Token::identify(&self.token)),
            }
        } else if ch.is_ascii_digit() {
            while ch.is_ascii_digit() {
                self.token.push(ch);
                if self.cur_index == self.chars.len() {
                    break;
                } else {
                    ch = self.chars[self.cur_index];
                    self.cur_index += 1;
                }
            }
            if self.cur_index != self.chars.len() {
                self.cur_index -= 1;
            }

            return Some(Token::unsigned_number(&self.token));
        } else if ch == ':' {
            if self.cur_index == self.chars.len() {
                Some(Token::delimiter("Colon"))
            } else {
                ch == self.chars[self.cur_index];
                self.cur_index += 1;
                match ch {
                    '=' => Some(Token::delimiter("Assign")),
                    _ => {
                        self.cur_index -= 1;
                        Some(Token::delimiter("Colon"))
                    }
                }
            }
        } else {
            match ch {
                '+' => Some(Token {
                    token_type: TokenType::Delimiter,
                    content: "Plus".to_string(),
                }),
                '*' => Some(Token {
                    token_type: TokenType::Delimiter,
                    content: "Star".to_string(),
                }),
                ',' => Some(Token {
                    token_type: TokenType::Delimiter,
                    content: "Comma".to_string(),
                }),
                '(' => Some(Token {
                    token_type: TokenType::Delimiter,
                    content: "LParenthesis".to_string(),
                }),
                ')' => Some(Token {
                    token_type: TokenType::Delimiter,
                    content: "RParenthesis".to_string(),
                }),
                _ => Some(Token {
                    token_type: TokenType::Unknown,
                    content: "".to_string(),
                })
            }
        };

        Some(Token {
            token_type: TokenType::Identify,
            content: self.token.clone(),
        })
    }
}
