use std::sync::OnceLock;

use crate::system::System;

pub mod memory;
pub mod simple_io;
pub mod stack;

pub static SYSTEM: OnceLock<System> = OnceLock::new();
