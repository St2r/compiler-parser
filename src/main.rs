use std::env::args;

fn main() {
    let input: Vec<String> = args().collect();
    let c = Content { content: &input[1] };
    for i in c {
        println!("{}", i.to_string());
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

// #[derive(Debug)]
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

    fn keyword_or_identify(value: &str) -> Token {
        return match value {
            "BEGIN" => Token::keyword("Begin"),
            "END" => Token::keyword("End"),
            "FOR" => Token::keyword("For"),
            "IF" => Token::keyword("If"),
            "THEN" => Token::keyword("Then"),
            "ELSE" => Token::keyword("Else"),
            _ => Token::identify(value),
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

impl Token {
    fn to_string(&self) -> String {
        match self.token_type {
            TokenType::Identify => {
                format!("Ident({})", self.content)
            }
            TokenType::Keyword => {
                format!("{}", self.content)
            }
            TokenType::UnsignedNumber => {
                format!("Int({})", self.content)
            }
            TokenType::Delimiter => {
                format!("{}", self.content)
            }
            TokenType::Unknown => {
                format!("Unknown")
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
                    return Some(Token::keyword_or_identify(&self.token))
                } else {
                    ch = self.chars[self.cur_index];
                    self.cur_index += 1;
                }
            }

            self.cur_index -= 1;

           Some(Token::keyword_or_identify(&self.token))
        } else if ch.is_ascii_digit() {
            while ch.is_ascii_digit() {
                self.token.push(ch);
                if self.cur_index == self.chars.len() {
                    return Some(Token::unsigned_number(&self.token))
                } else {
                    ch = self.chars[self.cur_index];
                    self.cur_index += 1;
                }
            }
            self.cur_index -= 1;

            return Some(Token::unsigned_number(&self.token));
        } else if ch == ':' {
            if self.cur_index == self.chars.len() {
                Some(Token::delimiter("Colon"))
            } else {
                ch = self.chars[self.cur_index];
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
    }
}

#[cfg(test)]
mod test_pascal_lex {
    use crate::*;

    #[test]
    fn test1() {
        let c = Content { content: "a1 a1 a1" };
        let mut iter: TokenIterator = c.into_iter();
        assert_eq!(iter.next().unwrap().to_string(), "Ident(a1)");
        assert_eq!(iter.next().unwrap().to_string(), "Ident(a1)");
        assert_eq!(iter.next().unwrap().to_string(), "Ident(a1)");

        match iter.next() {
            None => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test2() {
        let c = Content { content: ":= a2" };
        let mut iter: TokenIterator = c.into_iter();
        assert_eq!(iter.next().unwrap().to_string(), "Assign");
        assert_eq!(iter.next().unwrap().to_string(), "Ident(a2)");

        match iter.next() {
            None => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test3() {
        let c = Content { content: ":= a2 1a" };
        let mut iter: TokenIterator = c.into_iter();
        assert_eq!(iter.next().unwrap().to_string(), "Assign");
        assert_eq!(iter.next().unwrap().to_string(), "Ident(a2)");
        assert_eq!(iter.next().unwrap().to_string(), "Int(1)");
        assert_eq!(iter.next().unwrap().to_string(), "Ident(a)");

        match iter.next() {
            None => assert!(true),
            _ => assert!(false),
        }
    }
}