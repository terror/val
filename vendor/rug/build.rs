// Copyright © 2016–2026 Trevor Spiteri

// Copying and distribution of this file, with or without modification, are
// permitted in any medium without royalty provided the copyright notice and
// this notice are preserved. This file is offered as-is, without any warranty.

#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(gmp_limb_bits_32)");
    println!("cargo:rustc-check-cfg=cfg(gmp_limb_bits_64)");
    if env::var_os("CARGO_FEATURE_GMP_MPFR_SYS").is_some() {
        let bits =
            env::var_os("DEP_GMP_LIMB_BITS").expect("DEP_GMP_LIMB_BITS not set by gmp-mfpr-sys");
        let bits = bits
            .to_str()
            .expect("DEP_GMP_LIMB_BITS contains unexpected characters");
        if bits != "32" && bits != "64" {
            panic!("Limb bits not 32 or 64: \"{bits}\"");
        }
        println!("cargo:rustc-cfg=gmp_limb_bits_{bits}");
    }
}

#[allow(dead_code)]
fn check_feature(name: &str, contents: &str) {
    let rustc = cargo_env("RUSTC");
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));

    let try_dir = out_dir.join(format!("try_{name}"));
    let filename = format!("try_{name}.rs");
    create_dir_or_panic(&try_dir);
    println!("$ cd {try_dir:?}");
    create_file_or_panic(&try_dir.join(&filename), contents);
    let mut cmd = Command::new(rustc);
    cmd.current_dir(&try_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args([&*filename, "--emit=dep-info,metadata"]);
    println!("$ {cmd:?} >& /dev/null");
    let status = cmd
        .status()
        .unwrap_or_else(|_| panic!("Unable to execute: {cmd:?}"));
    println!("cargo:rustc-check-cfg=cfg({name})");
    if status.success() {
        println!("cargo:rustc-cfg={name}");
    }
    remove_dir_or_panic(&try_dir);
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {name}, please use cargo"))
}

fn remove_dir(dir: &Path) -> IoResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    assert!(dir.is_dir(), "Not a directory: {dir:?}");
    println!("$ rm -r {dir:?}");
    fs::remove_dir_all(dir)
}

fn remove_dir_or_panic(dir: &Path) {
    remove_dir(dir).unwrap_or_else(|_| panic!("Unable to remove directory: {dir:?}"));
}

fn create_dir(dir: &Path) -> IoResult<()> {
    println!("$ mkdir -p {dir:?}");
    fs::create_dir_all(dir)
}

fn create_dir_or_panic(dir: &Path) {
    create_dir(dir).unwrap_or_else(|_| panic!("Unable to create directory: {dir:?}"));
}

fn create_file_or_panic(filename: &Path, contents: &str) {
    println!("$ printf '%s' {:?}... > {:?}", &contents[0..10], filename);
    let mut file =
        File::create(filename).unwrap_or_else(|_| panic!("Unable to create file: {filename:?}"));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|_| panic!("Unable to write to file: {filename:?}"));
}
