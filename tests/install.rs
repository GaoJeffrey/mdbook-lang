use std::fs;
use std::process::Command;

use assert_cmd::prelude::*;

macro_rules! test_install {
    ($inputfile:expr, $msg:expr) => {
        let input = include_str!($inputfile);
        let expected = include_str!(concat!($inputfile, ".output"));

        let tmp = tempfile::tempdir().expect("can't create tempdir");

        let book_toml = tmp.path().join("book.toml");
        fs::write(&book_toml, input).expect("can't write book.toml");

        let mut cmd = Command::cargo_bin("mdbook-lang").unwrap();
        cmd.arg("install").current_dir(tmp.path());
        cmd.assert().success();

        let output = fs::read_to_string(&book_toml).expect("can't read book.toml");
        eprintln!("test_install_macro:\n{:}", output);
        pretty_assertions::assert_eq!(
            expected,
            output,
            "Mismatched data in {}: {}",
            $inputfile,
            $msg
        );

        assert!(
            tmp.path().join("lang.js").exists(),
            "Failed to copy lang.js"
        );
        
    };
}

#[test]
fn empty() {
    test_install!("empty.toml", "should add all configuration options");
}

#[test]
fn full() {
    test_install!("full.toml", "should leave it untouched");
}

#[test]
fn some() {
    test_install!("some.toml", "should add missing configuration options");
}

#[test]
fn missing_js() {
    test_install!("missing-js.toml", "should add missing javascript file");
}
