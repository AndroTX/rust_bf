#[derive(Debug)]
pub enum Symbol {
    Add,
    Sub,
    MoveLeft,
    MoveRight,
    LeftBracket,
    RightBracket,
    InputChar,
    OutputChar,

    Comment,
}

pub fn char_to_symbol(c: char) -> Symbol {
    return match c {
        '+' => Symbol::Add,
        '-' => Symbol::Sub,
        '>' => Symbol::MoveRight,
        '<' => Symbol::MoveLeft,
        '[' => Symbol::LeftBracket,
        ']' => Symbol::RightBracket,
        '.' => Symbol::OutputChar,
        ',' => Symbol::InputChar,

        _ => Symbol::Comment,
    }
}