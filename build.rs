use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=./scripts/");
    let output = Command::new("cmd")
        .args(["npm", "run", "build"])
        .output()
        .expect("failed to build scripts");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}
