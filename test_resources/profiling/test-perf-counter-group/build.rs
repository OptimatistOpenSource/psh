use std::fs;
use std::ops::Not;
use std::process::Command;

fn main() {
    let _ = fs::remove_file("src/bindings.rs");
    let mut cmd = Command::new("wit-bindgen");
    cmd.args(["rust", "--out-dir", "src", "wit"]);

    let output = cmd
        .output()
        .unwrap_or_else(|it| panic!("Failed to generate bindings: \n{}", it));
    if output.stderr.is_empty().not() {
        panic!(
            "Failed to generate bindings: \n{}",
            String::from_utf8(output.stderr).unwrap()
        );
    }
}
