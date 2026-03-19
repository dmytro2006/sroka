pub mod dfa;

use std::process::exit;
use crate::dfa::characters::parse_character;
use crate::dfa::State;

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

    fn build_token(prevState: State, buff: &mut String) -> Token{
       match prevState {
           State::BuildingDigit => {
               let skipped = buff.chars().filter(|c| *c != ' ').collect::<String>();
               Token::Number(skipped.parse::<i32>().unwrap())
           },
           State::BuildingIdentifier => Token::Identifier(buff.clone()),
           State::Plus => Token::Operator(Operator::Plus),
           State::Minus => Token::Operator(Operator::Minus),
           State::Slash => Token::Operator(Operator::Divide),
           State::OpenParentheses => Token::Parentheses(Parentheses::Open),
           State::CloseParentheses => Token::Parentheses(Parentheses::Close),
           State::Asterisk => Token::Operator(Operator::Multiply),
           _ => panic!("f"),
       }
    }
    fn skaner(self: &Self) -> Vec<Token> {
        let mut das = dfa::State::Start;
        let mut buff = String::new();

        let mut tokens = Vec::new();

        for i in 0..self.text.len() - 1{
            let curr_char = self.text.chars().nth(i).unwrap();
            let next_char = self.text.chars().nth(i + 1).unwrap();

            let current_character = parse_character(curr_char).unwrap();
            let next_character = parse_character(next_char).unwrap();

            das = dfa::transition(das, current_character);
            buff.push(curr_char);

            let try_next_das = dfa::transition(das, next_character);
            if try_next_das == dfa::State::End {
                let tok = Self::build_token(das,&mut buff);
                tokens.push(tok);
                buff.clear();
                das = dfa::State::Start;
            }
        }
        buff.push(self.text.chars().last().unwrap());
        das = dfa::transition(das, parse_character(self.text.chars().last().unwrap()).unwrap());

        let tok = Self::build_token(das,&mut buff);
        tokens.push(tok);


        //     match result {
        //         Ok(Some(token)) => tokens.push(token),
        //         Err(()) => {
        //             eprintln!("{}", self.text);
        //             for _ in 0..i {
        //                 eprint!(".");
        //             }
        //             eprint!("^ Tutaj\n");
        //
        //             eprintln!("Niedozwolony znak: '{}' na pozycji {}", a, i);
        //             exit(-1);
        //         }
        //         _ => {}
        //     }
        // }
        return tokens;
    }
}

fn main() {
    let sc = Scanner::new("2034     324+6 fadsf".to_owned());
    for token in sc.skaner() {
        println!("TOKEN: {:?}", token);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Operator, Parentheses, Scanner, Token};

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
