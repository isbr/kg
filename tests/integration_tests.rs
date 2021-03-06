extern crate assert_cli;

use assert_cli::Assert;
use std::fs::remove_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn teardown(dirname: &str) {
    if let Err(e) = remove_dir_all(dirname) {
        panic!("Error: {}", e.to_string());
    }
}

#[test]
fn basic() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/basic"])
        .unwrap();
    let txt_path = Path::new("_site/test.txt");
    assert!(txt_path.is_file());
    teardown("_site");
}

#[test]
fn markdown() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/markdown"])
        .unwrap();
    let html_path = Path::new("fixtures/out/markdown/cool.html");
    assert!(html_path.is_file());
    teardown("fixtures/out/markdown");
}

#[test]
fn markdown_options() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/markdown_options"])
        .unwrap();
    let mut html_file = File::open("fixtures/out/markdown_options/test.html").unwrap();
    let mut html_contents = String::new();
    html_file.read_to_string(&mut html_contents).unwrap();
    assert_eq!(html_contents, "<p>’”</p>\n");
    teardown("fixtures/out/markdown_options");
}

#[test]
fn layouts() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/layouts"])
        .unwrap();
    let layouts_path = Path::new("fixtures/out/layouts/_layouts/hello.hbs");
    assert!(!layouts_path.is_file());

    let mut html_file = File::open("fixtures/out/layouts/test.html").unwrap();
    let mut html_contents = String::new();
    html_file.read_to_string(&mut html_contents).unwrap();
    assert_eq!(html_contents, "test\n\ncool! hello\n");
    teardown("fixtures/out/layouts");
}

#[test]
fn ignore() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/ignore"])
        .unwrap();

    let html_path = Path::new("fixtures/out/ignore/cool.html");
    assert!(!html_path.is_file());
    teardown("fixtures/out/ignore");
}

#[test]
fn basic_handlebars() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/basic_handlebars"])
        .unwrap();

    let mut out_file = File::open("fixtures/out/basic_handlebars/test.html").unwrap();
    let mut html_contents = String::new();
    out_file.read_to_string(&mut html_contents).unwrap();
    assert_eq!(html_contents, "test site!\n");
    teardown("fixtures/out/basic_handlebars");
}

#[test]
fn i18n() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/i18n"])
        .unwrap();

    let de_path = Path::new("fixtures/out/i18n/de/main.html");
    let en_path = Path::new("fixtures/out/i18n/en/main.html");
    assert!(de_path.is_file());
    assert!(en_path.is_file());
    teardown("fixtures/out/i18n");
}

#[test]
fn filei18n() {
    Assert::main_binary()
        .with_args(&["build", "fixtures/filei18n"])
        .unwrap();

    let de_path = Path::new("fixtures/out/filei18n/de/cool.html");
    let untranslated_path = Path::new("fixtures/out/filei18n/normalfile.html");
    assert!(de_path.is_file());
    assert!(untranslated_path.is_file());
    teardown("fixtures/out/filei18n");
}
