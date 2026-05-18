use std::process::Command;

use insta::{assert_snapshot, glob, with_settings};
use insta_cmd::get_cargo_bin;

#[test]
fn examples() {
    with_settings!({
        prepend_module_to_snapshot => false,
        omit_expression => true,
    }, {
        glob!("../examples", "*.asm", |path| {
            let output = cli().arg(path).env("RUST_LOG", "debug").output().unwrap();
            assert_snapshot!(format!(
                "success: {}\nexit_code: {}\n----- stdout -----\n{}----- stderr -----\n{}",
                output.status.success(),
                output.status.code().unwrap(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        })
    })
}

fn cli() -> Command {
    Command::new(get_cargo_bin(env!("CARGO_PKG_NAME")))
}
