#![allow(dead_code)]

use nom::{branch::alt, character, AsChar, Parser};

use super::tokens::{Token, TokenKind};
use std::{error::Error, fmt, result};

const SINGLE_CHARACTERS: &'static str = "(){},.-+;/*";
// Contains information about error occurred during the parser execution
#[derive(Debug)]
pub struct ParseError {
    message: &'static str,
    source: Option<Box<dyn Error>>,
}

impl ParseError {
    pub fn new(message: &'static str, source: Option<Box<dyn Error>>) -> Self {
        Self { message, source }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[PARSE ERROR] {}", self.message)
    }
}

/// Represents the token parser signature for parsing language grammar. It takes the source string and build the corresponding token and return alongside the consumed characters amount.
///
/// Each language grammar parser should be a token parser.
pub type TokenParser<'a> = Box<dyn Parser<&'a str, (Token, usize), nom::error::Error<&'a str>>>;

fn single_char_token_parser<'a>() -> TokenParser<'a> {
    Box::new(|input: &'a str| {
        character::complete::one_of(SINGLE_CHARACTERS)
            .parse(input)
            .map(|(next, ch)| {
                let token = match ch {
                    '(' => Token::new(ch.to_string(), TokenKind::OpeningParentheses),
                    ')' => Token::new(ch.to_string(), TokenKind::ClosingParentheses),
                    '{' => Token::new(ch.to_string(), TokenKind::OpeningCurlyBrace),
                    '}' => Token::new(ch.to_string(), TokenKind::ClosingCurlyBrace),
                    ',' => Token::new(ch.to_string(), TokenKind::Comma),
                    '.' => Token::new(ch.to_string(), TokenKind::Dot),
                    '-' => Token::new(ch.to_string(), TokenKind::Minus),
                    '+' => Token::new(ch.to_string(), TokenKind::Plus),
                    ';' => Token::new(ch.to_string(), TokenKind::Semicolon),
                    '/' => Token::new(ch.to_string(), TokenKind::Slash),
                    '*' => Token::new(ch.to_string(), TokenKind::Star),

                    // characters were validated in parser, so this branch should be unreachable
                    _ => unreachable!(),
                };

                (next, (token, ch.len()))
            })
    })
}

/// Takes source code and performs the list of token parsers for available grammars.
///
/// If no one parser can parse the grammar, returns a parse error.
pub fn parse_token(input: &str) -> result::Result<(Token, usize), ParseError> {
    let (token, consumed) = alt((single_char_token_parser(),))
        .parse(input)
        .map(|(_, token)| token)
        .map_err(|_| ParseError::new("cannot parse", None))?;

    Ok((token, consumed))
}

#[cfg(test)]
mod tests {
    use nom::AsChar;

    use crate::tokenizer::tokens::{Token, TokenKind};

    use super::{single_char_token_parser, SINGLE_CHARACTERS};

    #[test]
    fn single_char_token_parser_test() {
        const INVALID_INPUT: &'static str = "invalid input";

        let expected_tokens = vec![
            Token::new('('.to_string(), TokenKind::OpeningParentheses),
            Token::new(')'.to_string(), TokenKind::ClosingParentheses),
            Token::new('{'.to_string(), TokenKind::OpeningCurlyBrace),
            Token::new('}'.to_string(), TokenKind::ClosingCurlyBrace),
            Token::new(','.to_string(), TokenKind::Comma),
            Token::new('.'.to_string(), TokenKind::Dot),
            Token::new('-'.to_string(), TokenKind::Minus),
            Token::new('+'.to_string(), TokenKind::Plus),
            Token::new(';'.to_string(), TokenKind::Semicolon),
            Token::new('/'.to_string(), TokenKind::Slash),
            Token::new('*'.to_string(), TokenKind::Star),
        ];

        for (i, ch) in SINGLE_CHARACTERS.chars().enumerate() {
            let (_, (token, consumed)) = single_char_token_parser()
                .parse(&SINGLE_CHARACTERS[i..])
                .unwrap();

            assert_eq!(
                token, expected_tokens[i],
                "should parse input and return a token for the respective lexeme"
            );
            assert_eq!(
                consumed,
                ch.len(),
                "should return consumed character length"
            )
        }

        let invalid_single_token = single_char_token_parser().parse(INVALID_INPUT);

        assert!(
            invalid_single_token.is_err(),
            "should return error if cannot parse single token"
        );
    }
}
