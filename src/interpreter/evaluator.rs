use std::any::Any;

use crate::{
    prelude::Result,
    tokens::{Token, TokenKind},
};

const CANNOT_EVALUATE_AS_NUMBER: &'static str = "cannot evaluate dynamic expression as number";
const CANNOT_EVALUATE_AS_BOOLEAN: &'static str = "cannot evaluate dynamic expression as boolean";
const CANNOT_DIVIDE_BY_ZERO: &'static str = "cannot divide by zero";
const INVALID_ARITHMETIC_OPERATION: &'static str = "invalid arithmetic operation";
const INVALID_COMPARISON_OPERATION: &'static str = "invalid comparison operation";
const INVALID_EQUALITY_OPERATION: &'static str = "invalid equality operation";

fn downcast_to_number(value: Box<dyn Any>) -> Result<f32> {
    let number = value
        .downcast_ref::<f32>()
        .ok_or(CANNOT_EVALUATE_AS_NUMBER)?;

    Ok(*number)
}

fn downcast_to_boolean(value: Box<dyn Any>) -> Result<bool> {
    let boolean = value
        .downcast_ref::<bool>()
        .ok_or(CANNOT_EVALUATE_AS_BOOLEAN)?;

    Ok(*boolean)
}

pub struct Evaluator;

impl Evaluator {
    /// Tries to downcast the given dynamic value as number and returns its negative version.
    pub fn negate_number(value: Box<dyn Any>) -> Result<Box<dyn Any>> {
        let number = downcast_to_number(value)?;
        Ok(Box::new(-number))
    }

    /// Tries to downcast the given dynamic value to boolean and returns its negated version.
    pub fn negate_boolean(value: Box<dyn Any>) -> Result<Box<dyn Any>> {
        let boolean = value
            .downcast_ref::<bool>()
            .ok_or(CANNOT_EVALUATE_AS_BOOLEAN)?;

        Ok(Box::new(!boolean))
    }

    /// Tries to downcast given dynamic values to numbers and performs arithmetic operation based on the provided token.
    pub fn compute_arithmetic(
        left_value: Box<dyn Any>,
        arithmetic_operator: Token,
        right_value: Box<dyn Any>,
    ) -> Result<Box<dyn Any>> {
        let left_number = downcast_to_number(left_value)?;
        let right_number = downcast_to_number(right_value)?;

        match arithmetic_operator.kind {
            TokenKind::Plus => Ok(Box::new(left_number + right_number)),
            TokenKind::Minus => Ok(Box::new(left_number - right_number)),
            TokenKind::Star => Ok(Box::new(left_number * right_number)),
            TokenKind::Slash => {
                if right_number == 0.0 {
                    return Err(CANNOT_DIVIDE_BY_ZERO.into());
                }

                Ok(Box::new(left_number / right_number))
            }

            _ => Err(INVALID_ARITHMETIC_OPERATION.into()),
        }
    }

    // TODO: maybe attach comparing traits to expected dynamic expressions to allow comparison between different types of values.
    /// Tries to downcast given dynamic values to numbers and performs comparison operation based on the provided token.
    pub fn compute_comparison(
        left_value: Box<dyn Any>,
        comparison_operator: Token,
        right_value: Box<dyn Any>,
    ) -> Result<Box<dyn Any>> {
        let left_number = downcast_to_number(left_value)?;
        let right_number = downcast_to_number(right_value)?;

        match comparison_operator.kind {
            TokenKind::Great => Ok(Box::new(left_number > right_number)),
            TokenKind::GreatEqual => Ok(Box::new(left_number >= right_number)),
            TokenKind::Less => Ok(Box::new(left_number < right_number)),
            TokenKind::LessEqual => Ok(Box::new(left_number <= right_number)),

            _ => Err(INVALID_COMPARISON_OPERATION.into()),
        }
    }

    // TODO: maybe attach comparing traits to expected dynamic expressions to allow comparison between different types of values.
    /// Tries to downcast given dynamic values to numbers and performs equality operation based on the provided token.
    pub fn compute_equality(
        left_value: Box<dyn Any>,
        equality_operator: Token,
        right_value: Box<dyn Any>,
    ) -> Result<Box<dyn Any>> {
        let left_number = downcast_to_number(left_value)?;
        let right_number = downcast_to_number(right_value)?;

        match equality_operator.kind {
            TokenKind::DoubleEqual => Ok(Box::new(left_number == right_number)),
            TokenKind::BangEqual => Ok(Box::new(left_number != right_number)),

            _ => Err(INVALID_EQUALITY_OPERATION.into()),
        }
    }
}
