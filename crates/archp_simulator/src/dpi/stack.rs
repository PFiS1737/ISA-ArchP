use std::sync::{LazyLock, Mutex};

pub struct Stack {
    data: Vec<u32>,
}

impl Stack {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn push(&mut self, value: u32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) {
        self.data.pop();
    }

    pub fn peek(&self) -> u32 {
        *self.data.last().unwrap_or(&0)
    }
}

static CALL_STACK: LazyLock<Mutex<Stack>> = LazyLock::new(|| Mutex::new(Stack::new()));

#[unsafe(no_mangle)]
extern "C" fn call_stack_pop() {
    CALL_STACK.lock().unwrap().pop();
}

#[unsafe(no_mangle)]
extern "C" fn call_stack_push(value: i32) {
    CALL_STACK.lock().unwrap().push(value as u32);
}

#[unsafe(no_mangle)]
extern "C" fn call_stack_peek() -> u32 {
    CALL_STACK.lock().unwrap().peek()
}

static DATA_STACK: LazyLock<Mutex<Stack>> = LazyLock::new(|| Mutex::new(Stack::new()));

#[unsafe(no_mangle)]
extern "C" fn data_stack_pop() {
    DATA_STACK.lock().unwrap().pop();
}

#[unsafe(no_mangle)]
extern "C" fn data_stack_push(value: i32) {
    DATA_STACK.lock().unwrap().push(value as u32);
}

#[unsafe(no_mangle)]
extern "C" fn data_stack_peek() -> u32 {
    DATA_STACK.lock().unwrap().peek()
}
