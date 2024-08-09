use std::{any::Any, fmt::Display};

mod evaluator;
pub mod runtime;

/// Wrapper over `Box<dyn Any>` to provide inject `Display` trait.
pub struct Output(Box<dyn Any>);

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = &self.0;

        if let Some(s) = output.downcast_ref::<String>() {
            write!(f, "{s}")?;
        } else if let Some(n) = output.downcast_ref::<f32>() {
            write!(f, "{n}")?;
        } else if let Some(b) = output.downcast_ref::<bool>() {
            write!(f, "{b}")?;
        }

        panic!("output cannot be printed")
    }
}
