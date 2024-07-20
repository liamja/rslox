use crate::token_type::TokenType;
use crate::token::Token;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token()
        }
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
            _ => eprintln!("Unexpected character: {c}"),
        }
    }

    fn advance(&mut self) -> char {
        let cur = self.current;
        self.current += 1;
        self.source.chars().nth(cur).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line))
    }
}
