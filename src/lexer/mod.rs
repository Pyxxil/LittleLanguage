use std::iter::Peekable;
use std::str::Chars;

pub mod token;
use token::Location;
use token::Token;

pub struct Lexer<'a> {
    input_iter: Peekable<Chars<'a>>,
    line_number: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            input_iter: content.chars().peekable(),
            line_number: 1,
            column: 1,
        }
    }

    pub fn lex(self) -> Vec<Token> {
        let tokens = Vec::from(self);
        let mut tokens = Vec::new();

        for token in self {
            tokens.push(token);
        }

        tokens
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input_iter.peek()
    }

    fn read_char(&mut self) -> Option<char> {
        self.column += 1;
        self.input_iter.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                let _ = self.read_char();
                if c == '\n' {
                    self.column = 1;
                    self.line_number += 1;
                }
            } else {
                break;
            }
        }
    }

    fn lookup_keyword(&mut self, identifier: String) -> Option<Token> {
        match identifier.as_ref() {
            "else" => Some(Token::Else(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            "container" => Some(Token::Container(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            "for" => Some(Token::For(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            "if" => Some(Token::If(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            "function" => Some(Token::Function(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            "variable" => Some(Token::Variable(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            "true" => Some(Token::True(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            "false" => Some(Token::False(Location::new(
                self.column - identifier.len(),
                self.line_number,
            ))),
            _ => Some(Token::Identifier(
                Location::new(self.column - identifier.len(), self.line_number),
                identifier,
            )),
        }
    }

    fn read_identifier(&mut self, ch: char) -> String {
        let mut identifier = String::new();
        identifier.push(ch);

        while let Some(&ch) = self.peek_char() {
            if Self::is_ident_character(ch) {
                identifier.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        identifier
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        let mut last = '\0';

        while let Some(&ch) = self.peek_char() {
            if (ch == '"') && (last != '\\') {
                let _ = self.read_char();
                break;
            } else {
                last = ch;
                string.push(self.read_char().unwrap());
            }
        }

        string
    }

    fn read_number(&mut self, ch: char) -> String {
        let mut number = String::new();
        number.push(ch);

        while let Some(&ch) = self.peek_char() {
            if ch.is_digit(10) {
                number.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        number
    }

    fn next_line(&mut self) {
        self.line_number += 1;
        self.column = 1;

        while let Some(&ch) = self.peek_char() {
            let _ = self.read_char();
            if ch == '\n' {
                break;
            }
        }
    }

    fn is_letter(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_ident_character(c: char) -> bool {
        Self::is_letter(c) || c.is_digit(10)
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let c = self.read_char();

        if let Some(ch) = c {
            match ch {
                '=' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::Equality(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::Assignment(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '>' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::GreaterEqualTo(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::GreaterThan(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '<' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::LessEqualTo(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::LessThan(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '-' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::SubtractEqual(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::Subtraction(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '+' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::AddEqual(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::Addition(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '*' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::MultiplyEqual(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::Multiply(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '/' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::DivideEqual(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else if let Some(&'/') = self.peek_char() {
                        self.next_line();
                        self.next_token()
                    } else {
                        Some(Token::Divide(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                ',' => Some(Token::Comma(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                '{' => Some(Token::OpenCurlyBrace(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                '}' => Some(Token::CloseCurlyBrace(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                '[' => Some(Token::OpenBracket(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                ']' => Some(Token::CloseBracket(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                '(' => Some(Token::OpenParentheses(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                ')' => Some(Token::CloseParentheses(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                ';' => Some(Token::Semicolon(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                '"' => Some(Token::StringLiteral(
                    Location::new(self.column - 1, self.line_number),
                    self.read_string(),
                )),
                '.' => Some(Token::Dot(Location::new(self.column - 1, self.line_number))),
                '~' => Some(Token::BitwiseNot(Location::new(
                    self.column - 1,
                    self.line_number,
                ))),
                '!' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::LogicalNotEqualTo(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::LogicalNot(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '&' => {
                    if let Some(&'&') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::LogicalAnd(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::BitwiseAnd(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                '|' => {
                    if let Some(&'|') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::LogicalOr(Location::new(
                            self.column - 2,
                            self.line_number,
                        )))
                    } else {
                        Some(Token::BitwiseOr(Location::new(
                            self.column - 1,
                            self.line_number,
                        )))
                    }
                }
                _ => {
                    if Self::is_letter(ch) {
                        let ident = self.read_identifier(ch);
                        self.lookup_keyword(ident)
                    } else if ch.is_digit(10) {
                        Some(Token::NumberLiteral(
                            Location::new(self.column, self.line_number),
                            self.read_number(ch),
                        ))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
