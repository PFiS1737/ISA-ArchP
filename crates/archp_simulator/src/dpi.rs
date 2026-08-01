mod memory;
mod register;
mod simple_io;
mod stack;

use std::sync::OnceLock;

use crate::system::System;

pub static SYSTEM: OnceLock<System> = OnceLock::new();
