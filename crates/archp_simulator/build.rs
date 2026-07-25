use std::{env, fs, path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VerilatorMakeConfig {
    version: i32,
    system: VerilatorMakeConfigSystem,
    options: VerilatorMakeConfigOptions,
    sources: VerilatorMakeConfigSources,
}

#[derive(Serialize, Deserialize)]
struct VerilatorMakeConfigSystem {
    verilator_root: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct VerilatorMakeConfigOptions {
    system_c: bool,
    coverage: bool,
    use_timing: bool,
    threads: i32,
    trace: bool,
    trace_fst: bool,
    trace_saif: bool,
    trace_vcd: bool,
}

#[derive(Serialize, Deserialize)]
struct VerilatorMakeConfigSources {
    global: Vec<PathBuf>,
    classes_slow: Vec<PathBuf>,
    classes_fast: Vec<PathBuf>,
    support_slow: Vec<PathBuf>,
    support_fast: Vec<PathBuf>,
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // ====================
    //     Veryl Build
    // ====================

    let veryl_out_dir = out_dir.join("veryl_out");
    if veryl_out_dir.exists() {
        fs::remove_dir_all(&veryl_out_dir).unwrap();
    }
    fs::create_dir_all(&veryl_out_dir).unwrap();

    let status = Command::new("veryl")
        .args(["build", "--quiet", "--out-dir"])
        .arg(&veryl_out_dir)
        .status()
        .unwrap();

    if !status.success() {
        panic!("Veryl build failed with status: {}", status);
    }

    println!("cargo:rerun-if-changed=veryl");

    // ====================
    //   Verilator Build
    // ====================

    let verilator_out_dir = out_dir.join("verilator_out");
    let v_prefix = "Vtop";

    let dpi_files = fs::read_dir("dpi").unwrap().flatten().filter_map(|entry| {
        let path = entry.path();
        if path.is_file() { Some(path) } else { None }
    });

    let status = Command::new("verilator")
        .args(["--cc", "--make", "json", "--prefix", v_prefix])
        .arg("--Mdir")
        .arg(&verilator_out_dir)
        .args([
            "-O2",
            "-Wall",
            "-Wno-DECLFILENAME", // Error: Filename 'bundled' does not match MODULE name: '...'
            "--x-assign",
            "fast",
            "--x-initial",
            "fast",
            "--no-assert",
        ])
        .args(dpi_files)
        .arg(veryl_out_dir.join("bundled.sv"))
        .status()
        .unwrap();

    if !status.success() {
        panic!("Verilator build failed with status: {}", status);
    }

    println!("cargo:rerun-if-changed=dpi");

    // ====================
    //       CC Build
    // ====================

    let config_file = format!("{}/{}.json", verilator_out_dir.display(), v_prefix);

    let config_data =
        fs::read_to_string(&config_file).expect("Failed to read Verilator JSON config");

    let config: VerilatorMakeConfig =
        serde_json::from_str(&config_data).expect("Failed to parse Verilator JSON config");

    cxx_build::bridge("src/cpu.rs")
        .std("c++20")
        .warnings(false)
        .define("VM_SC", if config.options.system_c { "1" } else { "0" })
        .define(
            "VM_COVERAGE",
            if config.options.coverage { "1" } else { "0" },
        )
        .define(
            "VM_TIMING",
            if config.options.use_timing { "1" } else { "0" },
        )
        .define("VM_THREADS", config.options.threads.to_string().as_str())
        .define("VM_TRACE", if config.options.trace { "1" } else { "0" })
        .define(
            "VM_TRACE_FST",
            if config.options.trace_fst { "1" } else { "0" },
        )
        .define(
            "VM_TRACE_SAIF",
            if config.options.trace_saif { "1" } else { "0" },
        )
        .define(
            "VM_TRACE_VCD",
            if config.options.trace_vcd { "1" } else { "0" },
        )
        .include(verilator_out_dir)
        .include(config.system.verilator_root.join("include"))
        .include(config.system.verilator_root.join("include/vltstd"))
        .files(config.sources.global)
        .files(config.sources.classes_slow)
        .files(config.sources.classes_fast)
        .files(config.sources.support_slow)
        .files(config.sources.support_fast)
        .file("cxx/cpu.cpp")
        .compile("archp_cpu");

    println!("cargo:rerun-if-changed=src/cpu.rs");
    println!("cargo:rerun-if-changed=cxx");
}
