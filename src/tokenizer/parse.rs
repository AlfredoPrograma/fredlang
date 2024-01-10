#![allow(dead_code)]

use nom::{branch::alt, character, AsChar, Parser};

use super::tokens::{Token, TokenKind};
use std::{error::Error, fmt, result};

const SINGLE_CHARACTERS: &'static str = "(){},.-+;/*\n";
const PAIR_COMPOSABLE_OPERATOR_CHARACTERS: &'static str = "!=<>";
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

impl Error for ParseError {}

/// Represents the token parser signature for parsing language grammar. It takes the source string and build the corresponding token and return alongside the consumed characters amount.
///
/// Each language grammar parser should be a token parser.
pub type TokenParser<'a> = Box<dyn Parser<&'a str, (Token, usize), nom::error::Error<&'a str>>>;

/// Tries to parse single token based on the grammar of the language.
///
/// Available single tokens are based on `SINGLE_CHARACTERS` list.
fn single_char_token_parser<'a>() -> TokenParser<'a> {
    Box::new(|input| {
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
                    '\n' => Token::new(ch.to_string(), TokenKind::EOF),

                    // characters were validated in parser, so this branch should be unreachable
                    _ => unreachable!(),
                };

                (next, (token, ch.len()))
            })
    })
}

/// Tries to parse single (but composable) operator token based on the grammar of the language.
///
/// Available single (but composable) operators tokens are based on `PAIR_COMPOSABLE_OPERATOR_CHARACTERS` list.
fn single_composable_operator_parser<'a>() -> TokenParser<'a> {
    Box::new(|input| {
        character::complete::one_of(PAIR_COMPOSABLE_OPERATOR_CHARACTERS)
            .parse(input)
            .map(|(next, ch)| {
                let token = match ch {
                    '!' => Token::new(ch.to_string(), TokenKind::Bang),
                    '=' => Token::new(ch.to_string(), TokenKind::Equal),
                    '>' => Token::new(ch.to_string(), TokenKind::Greater),
                    '<' => Token::new(ch.to_string(), TokenKind::Lesser),

                    // characters were validated in parser, so this branch should be unreachable
                    _ => unreachable!(),
                };

                (next, (token, ch.len()))
            })
    })
}

/// Tries to parse composed operator token based on the grammar of the language.
fn pair_composable_operator_parser<'a>() -> TokenParser<'a> {
    Box::new(|input| {
        // first find the operator and try to parse it
        character::complete::one_of(PAIR_COMPOSABLE_OPERATOR_CHARACTERS)
            .parse(input)
            // then uses the captured operator and tries to match with `=` as the next character
            .and_then(|(next, first)| {
                character::complete::char('=')
                    .parse(next)
                    .map(|(next, second)| {
                        let pair = format!("{first}{second}");

                        let token = match pair.as_str() {
                            "!=" => Token::new(pair.to_string(), TokenKind::BangEqual),
                            "==" => Token::new(pair.to_string(), TokenKind::DoubleEqual),
                            ">=" => Token::new(pair.to_string(), TokenKind::GreaterEqual),
                            "<=" => Token::new(pair.to_string(), TokenKind::LesserEqual),

                            // characters were validated in parser, so this branch should be unreachable
                            _ => unreachable!(),
                        };

                        (next, (token, pair.len()))
                    })
            })
    })
}

/// Takes source code and performs the list of token parsers for available grammars.
///
/// If no one parser can parse the grammar, returns a parse error.
pub fn parse_token(input: &str) -> result::Result<(Token, usize), ParseError> {
    let (token, consumed) = alt((
        pair_composable_operator_parser(),
        single_char_token_parser(),
        single_composable_operator_parser(),
    ))
    .parse(input)
    .map(|(_, token)| token)
    .map_err(|_| ParseError::new("cannot parse", None))?;

    Ok((token, consumed))
}

#[cfg(test)]
mod tests {
    use nom::AsChar;

    use crate::tokenizer::{
        parse::{pair_composable_operator_parser, single_composable_operator_parser},
        tokens::{Token, TokenKind},
    };

    use super::{single_char_token_parser, SINGLE_CHARACTERS};

    #[test]
    fn try_single_char_token_parser() {
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
            Token::new('\n'.to_string(), TokenKind::EOF),
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

    #[test]
    fn try_single_composable_operator_parser<'a>() {
        const SINGLE_OPERATORS: &'static str = "!=><";

        let expected_tokens = vec![
            Token::new('!'.to_string(), TokenKind::Bang),
            Token::new('='.to_string(), TokenKind::Equal),
            Token::new('>'.to_string(), TokenKind::Greater),
            Token::new('<'.to_string(), TokenKind::Lesser),
        ];

        for (i, op) in SINGLE_OPERATORS.chars().enumerate() {
            let (_, (token, consumed)) = single_composable_operator_parser()
                .parse(&SINGLE_OPERATORS[i..])
                .unwrap();

            assert_eq!(
                token, expected_tokens[i],
                "should parse input and return a token for the respective operator"
            );

            assert_eq!(
                consumed,
                op.len(),
                "should return consumed characters length"
            )
        }
    }

    #[test]
    fn try_pair_composable_operator_parser<'a>() {
        const COMPOSED_OPERATORS: &'static str = "!===>=<=";

        let expected_tokens = vec![
            Token::new("!=".to_string(), TokenKind::BangEqual),
            Token::new("==".to_string(), TokenKind::DoubleEqual),
            Token::new(">=".to_string(), TokenKind::GreaterEqual),
            Token::new("<=".to_string(), TokenKind::LesserEqual),
        ];

        let mut cursor = 0;
        let mut expected_token_index = 0;

        while cursor < COMPOSED_OPERATORS.len() {
            let (_, (token, consumed)) = pair_composable_operator_parser()
                .parse(&COMPOSED_OPERATORS[cursor..])
                .unwrap();

            assert_eq!(
                token, expected_tokens[expected_token_index],
                "should parse input and return a token for the respective operator"
            );

            assert_eq!(
                consumed,
                token.lexeme.len(),
                "should return consumed characters length"
            );

            cursor += consumed;
            expected_token_index += 1;
        }
    }
}
