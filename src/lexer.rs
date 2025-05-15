//! # Lexer
//!
//! Converts the contents of a file into lexical tokens that can be converted
//! into an ABT.

/// Defines a BF token from
#[derive(Debug, PartialEq)]
pub enum Token {
    /// '+' symbol
    Increment,
    /// '-' symbol
    Decrement,
    /// '<' symbol
    MoveLeft,
    /// '>' symbol
    MoveRight,
    /// ',' symbol
    Read,
    /// '.' symbol
    Write,
    /// '[' symbol
    LoopStart,
    /// ']' symbol
    LoopEnd,
}

/// Tokenizes an input stream of characters into a stream of `Token` values
pub fn tokenize(input: &str) -> Vec<Token> {
    // Make peekable so that we can see characters ahead of current position
    // without consuming them
    let mut iter = input.chars().peekable();

    // Skip any leading whitespace and comment loops at beginning of the
    // program.
    loop {
        // skip whitespace
        while matches!(iter.peek(), Some(c) if c.is_whitespace()) {
            iter.next();
        }
        // if it’s a '[' then treat it as a comment‑loop and skip until its
        // matching ']'
        if let Some('[') = iter.peek() {
            let mut depth = 0;
            for ch in iter.by_ref() {
                match ch {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            // check for additional leading comment loop
            continue;
        }
        // past the prelude — stop skipping
        break;
    }

    // filter BF tokens from the remainder
    iter.filter_map(|c| match c {
        '+' => Some(Token::Increment),
        '-' => Some(Token::Decrement),
        '<' => Some(Token::MoveLeft),
        '>' => Some(Token::MoveRight),
        ',' => Some(Token::Read),
        '.' => Some(Token::Write),
        '[' => Some(Token::LoopStart),
        ']' => Some(Token::LoopEnd),
        _ => None,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two() {
        let program = "+[->+<]";
        assert_eq!(
            vec![
                Token::Increment,
                Token::LoopStart,
                Token::Decrement,
                Token::MoveRight,
                Token::Increment,
                Token::MoveLeft,
                Token::LoopEnd
            ],
            tokenize(program)
        )
    }

    #[test]
    fn non_bf_only() {
        let program = "\
            adipisicing aliqua velit et cupidatat velit consectetur \
            exercitation voluptate voluptate ut id elit voluptate \
            ullamco";
        assert_eq!(Vec::<Token>::new(), tokenize(program))
    }

    #[test]
    fn comment_only() {
        let program = "\
            [ \
            In BF you could write comments within a loop at the start of the\
            program since it will continue immediate since the current cell for\
            the start of the loop is initially 0\
            ]";
        assert_eq!(Vec::<Token>::new(), tokenize(program))
    }
}
