pub enum Character {
    Digit,
    Plus,
    Minus,
    Slash, // not backslash
    WhiteSpace,
    OpenParentheses,
    CloseParentheses,
    Letter,
    Asterisk,
}

pub fn parse_character(c: char) -> Option<Character> {
    match c {
        '0'..='9' => Some(Character::Digit),
        'a'..='z' | 'A'..='Z' => Some(Character::Letter),
        '+' => Some(Character::Plus),
        '-' => Some(Character::Minus),
        '/' => Some(Character::Slash),
        '(' => Some(Character::OpenParentheses),
        ')' => Some(Character::CloseParentheses),
        '*' => Some(Character::Asterisk),
        ' ' => Some(Character::WhiteSpace),
        _ => None,
    }
}
