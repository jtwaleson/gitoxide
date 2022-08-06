//! Parse [ref specs]() and represent them.
#![forbid(unsafe_code, rust_2018_idioms)]
#![allow(missing_docs)]

pub mod parse;
pub use parse::function::parse;

/// A refspec with references to the memory it was parsed from.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct RefSpecRef<'a> {
    mode: Mode,
    op: Operation,
    src: Option<&'a bstr::BStr>,
    dest: Option<&'a bstr::BStr>,
}

/// An owned refspec.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct RefSpec {
    mode: Mode,
    op: Operation,
    src: Option<bstr::BString>,
    dest: Option<bstr::BString>,
}

mod spec;

mod types;
pub use types::{Fetch, Instruction, Mode, Operation, Push};
