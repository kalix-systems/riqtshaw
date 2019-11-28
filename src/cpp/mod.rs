//! `cpp` is the module that generates the cpp code for the bindings

use crate::configuration::*;
use crate::util::{snake_case, write_if_different};
use std::io::{Result, Write};

mod header;
pub use header::write_header;

mod code;
pub use code::write_cpp;

mod codegen;
use self::codegen::Block;

mod helpers;
use helpers::*;
