use std::process::Command;

fn main() {
    generate_hermes();
    build_hermes();
}

fn generate_hermes() {
    assert!(Command::new("cmake")
        .args([
            "-S",
            "hermes",
            "-B",
            "build_release",
            "-G",
            "Ninja",
            "-D",
            "CMAKE_BUILD_TYPE=Release",
        ])
        .status()
        .unwrap()
        .success());
}

fn build_hermes() {
    assert!(Command::new("cmake")
        .args(["--build", "./build_release"])
        .status()
        .unwrap()
        .success());
}
