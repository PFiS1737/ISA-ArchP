mod memory;
mod register;
mod stack;
mod syscall;

use std::sync::OnceLock;

use crate::system::System;

pub static SYSTEM: OnceLock<System> = OnceLock::new();
