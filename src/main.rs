use crate::State::Start;

#[derive(Debug)]
#[derive(PartialEq)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Parentheses{
    Open,
    Close
}

#[repr(u8)]
#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Number(i32) = 0,
    Operator(Operator),
    Identifier(String),
    Parentheses(Parentheses)
}

enum State {
    Start,
    Q1,
    Q2,
}

struct DAS {
    current: State,
    number_buffer: i32,
    id_buffer: String,
}

fn as_digit(ch: char) -> i32 {
    ch as i32 - '0' as i32
}

fn is_digit(ch: char) -> bool {
    let digit = as_digit(ch);
    0 <= digit && digit <= 9
}

fn is_variablable(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        '_' => true,
        'a'..='z' => true,
        _ => false,
    }
}

impl DAS {
    fn new() -> Self {
        Self {
            current: Start,
            number_buffer: 0,
            id_buffer: String::new(),
        }
    }
    fn update(self: &mut Self, input: char, next: char) -> Option<Token> {
        match &mut self.current {
            Start => {
                self.number_buffer = 0;
                self.id_buffer.clear();
                match input {
                    '0'..='9' => {
                        self.number_buffer = self.number_buffer * 10 + as_digit(input);
                        if !is_digit(next) {
                            return Some(Token::Number(self.number_buffer));
                        }
                        self.current = State::Q1;
                    }
                    '+' => return Some(Token::Operator(Operator::Plus)),
                    '-' => return Some(Token::Operator(Operator::Minus)),
                    '*' => return Some(Token::Operator(Operator::Multiply)),
                    '/' => return Some(Token::Operator(Operator::Divide)),
                    '(' => return Some(Token::Parentheses(Parentheses::Open)),
                    ')' => return Some(Token::Parentheses(Parentheses::Close)),
                    'a'..'z' => {
                        self.id_buffer.push(input);
                        if !is_variablable(next) {
                            return Some(Token::Identifier(self.id_buffer.clone()));
                        }
                        self.current = State::Q2;
                    }
                    ' ' => {
                    }
                    _ => {
                        panic!("blad")
                    }
                }
            }
            State::Q1 => match input {
                '0'..='9' => {
                    self.number_buffer = self.number_buffer * 10 + as_digit(input);
                    if !is_digit(next) {
                        self.current = State::Start;
                        return Some(Token::Number(self.number_buffer));
                    }
                }
                _ => {}
            },
            State::Q2 => match input {
                '0'..='9' | 'a'..='z' | '_' => {
                    self.id_buffer.push(input);
                    if !is_variablable(next) {
                        self.current = State::Start;
                        return Some(Token::Identifier(self.id_buffer.clone()));
                    }
                }
                _ => {}
            },
        }
        return None;
    }
}

struct Scanner {
    text: String,
}

impl Scanner {
    fn new(text: String) -> Self {
        let mut t = text;
        t.push('!');
        Scanner {
            text: t,
        }
    }
    fn skaner(self: &Self) -> Vec<Token> {
        let mut das = DAS::new();
        let mut tokens = Vec::new();
        for i in 0..self.text.len() - 1 {
            let token = das.update(
                self.text.chars().nth(i).unwrap(),
                self.text.chars().nth(i + 1).unwrap(),
            );
            if let Some(token) = token {
                tokens.push(token);
            }
        }
        return tokens;
    }
}

fn main() {
    let sc = Scanner::new("f)".to_owned());
    for token in sc.skaner() {
        println!("TOKEN: {:?}", token);
    }
}

#[cfg(test)]
mod tests{
    use crate::{Scanner, Token, Operator, Parentheses};

    #[test]
    fn test_numbers_and_plus() {
        let sc = Scanner::new("1+2+3".to_owned());

        let tokens = sc.skaner();

        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Operator(Operator::Plus),
                Token::Number(2),
                Token::Operator(Operator::Plus),
                Token::Number(3),
            ]
        );
    }

    #[test]
    fn test_identifier_and_number() {
        let sc = Scanner::new("abc+12".to_owned());

        let tokens = sc.skaner();

        assert_eq!(
            tokens,
            vec![
                Token::Identifier("abc".to_string()),
                Token::Operator(Operator::Plus),
                Token::Number(12),
            ]
        );
    }

    #[test]
    fn test_parentheses_and_multiply() {
        let sc = Scanner::new("(a*3)".to_owned());

        let tokens = sc.skaner();

        assert_eq!(
            tokens,
            vec![
                Token::Parentheses(Parentheses::Open),
                Token::Identifier("a".to_string()),
                Token::Operator(Operator::Multiply),
                Token::Number(3),
                Token::Parentheses(Parentheses::Close),
            ]
        );
    }
}