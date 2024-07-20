use std::collections::HashMap;
use crate::token::Token;
use crate::token_type::TokenType;

pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token()
        }
        
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), "".to_string(), self.line));
        self.tokens.as_ref()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, c.to_string()),
            ')' => self.add_token(TokenType::RightParen, c.to_string()),
            '{' => self.add_token(TokenType::LeftBrace, c.to_string()),
            '}' => self.add_token(TokenType::RightBrace, c.to_string()),
            ',' => self.add_token(TokenType::Comma, c.to_string()),
            '.' => self.add_token(TokenType::Dot, c.to_string()),
            '-' => self.add_token(TokenType::Minus, c.to_string()),
            '+' => self.add_token(TokenType::Plus, c.to_string()),
            ';' => self.add_token(TokenType::Semicolon, c.to_string()),
            '*' => self.add_token(TokenType::Star, c.to_string()),

            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token_type, c.to_string())
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type, c.to_string())
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token_type, c.to_string())
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token_type, c.to_string())
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    // Comments are lexemes, but they aren’t meaningful, and the parser doesn’t want
                    // to deal with them. So when we reach the end of the comment, we don’t call addToken().
                    // When we loop back around to start the next lexeme, start gets reset
                    // and the comment’s lexeme disappears in a puff of smoke.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, c.to_string())
                };
            }

            ' ' | '\r' | '\t' => {} // Ignore whitespace.

            '\n' => self.line += 1, // Ignore newline, but still increment the line counter.

            '"' => self.string(),

            '0'..='9' => self.number(),

            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            _ => eprintln!("Unexpected character: {c}"),
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        
        // Check if the identifier is a reserved keyword.
        let token_type = self.keywords.get(&text).map_or_else(|| TokenType::Identifier, |t| t.clone());
        
        self.add_token(token_type, text);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the ".".
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number, self.source[self.start..self.current].to_string())
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                // We still need to increment the line counter if we encounter a newline character.
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // todo: Lox.error(line, "Unterminated string.");
            eprintln!("Unterminated string on line {}.", self.line);
            return;
        }

        // We've reached the closing ".
        self.advance();

        // Trim the surrounding quotes from the discovered string literal.
        let value = self.source[self.start + 1..self.current - 1].to_string();
        // If Lox supported escape sequences like \n, we’d unescape those here.
        // todo: Support escape sequences.
        self.add_token(TokenType::String, value);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
            _ => false,
        }
    }

    fn advance(&mut self) -> char {
        let cur = self.current;
        self.current += 1;
        self.source.chars().nth(cur).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }
}
