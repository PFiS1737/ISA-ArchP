use std::io::{Write, stdin, stdout};

#[unsafe(no_mangle)]
extern "C" fn simple_input() -> i32 {
    print!("Input: ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim()
        .parse::<i32>()
        .expect("Failed to parse input as i32")
}

#[unsafe(no_mangle)]
extern "C" fn simple_output(value: i32) {
    println!("Output: {}", value);
}
