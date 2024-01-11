#![allow(dead_code)]

use nom::{branch::alt, bytes, character, number, sequence, AsChar, Parser};

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

/// Tries to parse literal (string) token based on the grammar of the language.
fn literal_parser<'a>() -> TokenParser<'a> {
    const LITERAL_DELIMITER: char = '"';
    Box::new(|input: &'a str| {
        sequence::delimited(
            character::complete::char(LITERAL_DELIMITER),
            // TODO: handle break lines inside literal strings
            bytes::complete::take_until(LITERAL_DELIMITER.to_string().as_str()),
            character::complete::char(LITERAL_DELIMITER),
        )
        .parse(input)
        .map(|(next, literal)| {
            let token = Token::new(literal.to_string(), TokenKind::Literal);

            // `consumed` amount is equivalent to the length of the parsed literal string plus the
            // delimiters sizes
            let consumed = literal.len() + (LITERAL_DELIMITER.len() * 2);

            (next, (token, consumed))
        })
    })
}

/// Tries to parse a number token based on the grammar of the language.
fn number_parser<'a>() -> TokenParser<'a> {
    Box::new(|input| {
        number::complete::double(input).map(|(next, number)| {
            let token = Token::new(number.to_string(), TokenKind::Number);

            (next, (token, number.to_string().len()))
        })
    })
}

/// Takes source code and performs the list of token parsers for available grammars.
///
/// If no one parser can parse the grammar, returns a parse error.
pub fn parse_token(input: &str) -> result::Result<(Token, usize), ParseError> {
    let (token, consumed) = alt((
        literal_parser(),
        number_parser(),
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
        parse::{
            literal_parser, pair_composable_operator_parser, single_composable_operator_parser,
        },
        tokens::{Token, TokenKind},
    };

    use super::{number_parser, single_char_token_parser, SINGLE_CHARACTERS};

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

    #[test]
    fn try_literal_parser() {
        const SOURCE: &'static str = "\"this is a string\"";
        let expected_token = Token::new("this is a string".to_string(), TokenKind::Literal);

        let (_, (token, consumed)) = literal_parser().parse(SOURCE).unwrap();

        assert_eq!(
            token, expected_token,
            "should parse input and return a token for literal"
        );

        assert_eq!(
            consumed,
            SOURCE.len(),
            "should return corresponding consumed characters length"
        )
    }

    #[test]
    fn try_number_parser() {
        let source_numbers = vec!["0", "-10", "-5.25", "20", "29.55", "3.05"];
        let expected_tokens: Vec<Token> = source_numbers
            .clone()
            .into_iter()
            .map(|n| Token::new(n.to_string(), TokenKind::Number))
            .collect();

        for (i, number) in source_numbers.clone().into_iter().enumerate() {
            let (_, (token, consumed)) = number_parser().parse(number).unwrap();

            assert_eq!(
                token, expected_tokens[i],
                "should parse input and return the corresponding number token"
            );

            assert_eq!(
                consumed,
                number.len(),
                "should return corresponding consumed characters length"
            )
        }
    }
}
