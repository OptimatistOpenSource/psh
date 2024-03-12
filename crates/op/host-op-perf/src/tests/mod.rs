use std::process::Command;

mod counter;
mod counter_group;

pub fn compile_component(project_path: &str) {
    let toml_path = format!("{}/Cargo.toml", project_path);

    // cargo clean --manifest-path $project_path/Cargo.toml
    let mut cmd = Command::new("cargo");
    cmd.args(["clean", "--manifest-path", &toml_path]);
    cmd.output().unwrap();

    // cargo component build --manifest-path $project_path/Cargo.toml
    let mut cmd = Command::new("cargo");
    cmd.args(["component", "build", "--manifest-path", &toml_path]);
    cmd.output().unwrap();
}
