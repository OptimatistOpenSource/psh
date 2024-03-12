use std::process::Command;

fn main() {
    // update git submodule
    Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("Failed to update git submodule");
}
