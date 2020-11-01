#![warn(rust_2018_idioms)]
#[macro_use]
extern crate lazy_static;

mod context;

pub use crate::context::{
    Context,
    ContextKey,
    ContextValue,
};
