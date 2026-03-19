use characters::Character;

pub mod characters;


#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum State{
    Start,
    End,
    BuildingDigit,
    BuildingIdentifier,

    Plus,
    Minus,
    Slash, // not backslash
    OpenParentheses,
    CloseParentheses,
    Asterisk,
}

pub fn is_accepting(c: State) -> bool {
    match(c) {
        State::Start => false,
        State::BuildingDigit => true,
        State::BuildingIdentifier => true,
        State::Plus => true,
        State::Minus => true,
        State::Slash => true,
        State::OpenParentheses => true,
        State::CloseParentheses => true,
        State::Asterisk => true,
        State::End => false,
    }
}

fn transition_start(c: Character) -> State{
    match c{
        Character::Digit => State::BuildingDigit,
        Character::Plus => State::Plus,
        Character::Minus => State::Minus,
        Character::Slash => State::Slash,
        Character::OpenParentheses => State::OpenParentheses,
        Character::CloseParentheses => State::CloseParentheses,
        Character::Letter => State::BuildingIdentifier,
        Character::Asterisk => State::Asterisk,
        Character::WhiteSpace => State::Start,
    }
}

fn transition_digit(c: Character) -> State{
    match c{
        Character::Digit => State::BuildingDigit,
        _ => State::End,
    }
}

fn transition_building_identifier(c: Character) -> State{
    match c{
        Character::Digit | Character::Letter => State::BuildingIdentifier,
        _ => State::End,
    }
}

pub fn transition(s: State, c: Character) -> State{
    match s{
        State::Start => transition_start(c),
        State::BuildingDigit => transition_digit(c),
        State::BuildingIdentifier => transition_building_identifier(c),
        State::Plus => State::End,
        State::Minus => State::End,
        State::Slash => State::End,
        State::OpenParentheses => State::End,
        State::CloseParentheses => State::End,
        State::Asterisk => State::End,
        State::End => State::End,
    }
}