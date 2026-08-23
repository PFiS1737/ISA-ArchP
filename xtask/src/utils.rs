use colored::Colorize;

pub fn print_h1(s: &str) {
    println!("{}", s.blue());
    println!("{}", "=".repeat(s.len()).blue());
}

pub fn print_error(s: &str) {
    println!("{}", s.red());
}
