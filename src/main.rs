pub mod dfa;

use crate::dfa::State;
use crate::dfa::characters::parse_character;
use std::process::exit;

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
enum Parentheses {
    Open,
    Close,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum Token {
    Number(i32) = 0,
    Operator(Operator),
    Identifier(String),
    Parentheses(Parentheses),
}

struct Scanner {
    text: String,
}

impl Scanner {
    fn new(text: String) -> Self {
        Scanner { text }
    }

    fn build_token(prev_state: State, buff: &mut String) -> Token {
        match prev_state {
            State::BuildingDigit => {
                let skipped = buff.chars().filter(|c| *c != ' ').collect::<String>();
                Token::Number(skipped.parse::<i32>().unwrap())
            }
            State::BuildingIdentifier => Token::Identifier(buff.clone()),
            State::Plus => Token::Operator(Operator::Plus),
            State::Minus => Token::Operator(Operator::Minus),
            State::Slash => Token::Operator(Operator::Divide),
            State::OpenParentheses => Token::Parentheses(Parentheses::Open),
            State::CloseParentheses => Token::Parentheses(Parentheses::Close),
            State::Asterisk => Token::Operator(Operator::Multiply),
            _ => unreachable!(),
        }
    }

    fn error(text: &str, pos: usize) {
        eprintln!("{}", text);
        for _ in 0..pos {
            eprint!(".");
        }
        eprint!("^ Tutaj\n");

        eprintln!(
            "Niedozwolony znak: '{}' na pozycji {}",
            text.chars().nth(pos).unwrap(),
            pos
        );
        exit(-1);
    }
    fn scan(self: &Self) -> Vec<Token> {
        let mut das = dfa::State::Start;
        let mut buff = String::new();

        let mut tokens = Vec::new();

        for i in 0..self.text.len() - 1 {
            let curr_char = self.text.chars().nth(i).unwrap();
            let next_char = self.text.chars().nth(i + 1).unwrap();

            let current_character = parse_character(curr_char).unwrap_or_else(|| {
                Self::error(self.text.as_str(), i);
                exit(-1);
            });
            let next_character = parse_character(next_char).unwrap_or_else(|| {
                Self::error(self.text.as_str(), i + 1);
                exit(-1);
            });

            das = dfa::transition(das, current_character);
            buff.push(curr_char);

            let try_next_das = dfa::transition(das, next_character);
            if try_next_das == dfa::State::End {
                let tok = Self::build_token(das, &mut buff);
                tokens.push(tok);
                buff.clear();
                das = dfa::State::Start;
            }
        }
        let last_char = self.text.chars().last().unwrap();
        let last_character = parse_character(last_char).unwrap_or_else(|| {
            Self::error(self.text.as_str(), self.text.len() - 1);
            exit(-1);
        });
        buff.push(last_char);
        das = dfa::transition(das, last_character);

        let tok = Self::build_token(das, &mut buff);
        tokens.push(tok);

        return tokens;
    }
}

fn main() {
    let sc = Scanner::new("2034     324+6 fad;sf".to_owned());
    for token in sc.scan() {
        println!("TOKEN: {:?}", token);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Operator, Parentheses, Scanner, Token};

    #[test]
    fn test_numbers_and_plus() {
        let sc = Scanner::new("1+2+3".to_owned());

        let tokens = sc.scan();

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

        let tokens = sc.scan();

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

        let tokens = sc.scan();

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
