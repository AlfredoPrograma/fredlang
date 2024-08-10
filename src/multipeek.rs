use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug)]
pub struct Multipeek<I>
where
    I: Iterator,
    I::Item: Debug + Clone,
{
    iter: I,
    peeked: VecDeque<Option<I::Item>>,
    current: Option<I::Item>,
}

impl<I> Multipeek<I>
where
    I: Iterator,
    I::Item: Debug + Clone,
{
    pub fn new(mut iter: I) -> Self {
        let current = iter.next();
        let consumed = VecDeque::from(vec![current.clone()]);

        Self {
            current,
            peeked: consumed,
            iter,
        }
    }
}

impl<I> Iterator for Multipeek<I>
where
    I: Iterator,
    I::Item: Debug + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(consumed) = self.peeked.pop_back() {
            return consumed;
        }

        self.iter.next()
    }
}

pub trait Multipeeker {
    type Item;

    fn peek(&mut self) -> Option<Self::Item>;
}

impl<I> Multipeeker for Multipeek<I>
where
    I: Iterator,
    I::Item: Debug + Clone,
{
    type Item = I::Item;
    fn peek(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        self.peeked.push_front(next.clone());

        next
    }
}

#[cfg(test)]
mod multipeek_tests {
    use crate::{
        multipeek::Multipeeker,
        tokens::{Token, TokenKind},
    };

    use super::Multipeek;

    #[test]
    fn create_multipeek() {
        let tokens = vec![
            Token::new("3".to_string(), TokenKind::Number, 1),
            Token::new("*".to_string(), TokenKind::Star, 1),
            Token::new("5".to_string(), TokenKind::Number, 1),
            Token::new("/".to_string(), TokenKind::Slash, 1),
            Token::new("2".to_string(), TokenKind::Number, 1),
        ];
        let multipeek = Multipeek::new(tokens.clone().into_iter());

        assert_eq!(
            multipeek.current,
            Some(tokens[0].clone()),
            "current field should start at first element of the iterator"
        );
    }

    #[test]
    fn consume_next() {
        let tokens = vec![
            Token::new("3".to_string(), TokenKind::Number, 1),
            Token::new("*".to_string(), TokenKind::Star, 1),
            Token::new("5".to_string(), TokenKind::Number, 1),
            Token::new("/".to_string(), TokenKind::Slash, 1),
            Token::new("2".to_string(), TokenKind::Number, 1),
        ];
        let multipeek = Multipeek::new(tokens.clone().into_iter());

        for (i, token) in multipeek.enumerate() {
            assert_eq!(
                token, tokens[i],
                "tokens on multipeek iteration should be same as expected"
            )
        }
    }

    #[test]
    fn peek_multiple_times() {
        let numbers = Vec::from_iter(0..10);

        let mut multipeek = Multipeek::new(numbers.into_iter());

        for n in 0..4 {
            assert!(multipeek.peek().is_some_and(|v| v == n + 1));
        }

        for (i, n) in multipeek.enumerate() {
            assert!(i == n);
        }
    }
}
