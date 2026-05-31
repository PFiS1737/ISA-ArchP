use std::{env, process::Command};

use insta::{assert_snapshot, glob, with_settings};

#[test]
fn examples() {
    with_settings!({
        prepend_module_to_snapshot => false,
        omit_expression => true,
    }, {
        glob!("../../archp_assembler/examples", "*.asm", |path| {
            let output = cli().arg(path).args(["--hex", "--stdout"]).env("RUST_LOG", "debug").output().unwrap();
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
    let exe_path = env!("CARGO_BIN_EXE_archp-as");
    Command::new(exe_path)
}
