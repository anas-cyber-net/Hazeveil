use std::process::Command;

fn main() {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to get kernel version");

    let kernel = String::from_utf8_lossy(&output.stdout);
    let kernel = kernel.trim();

    println!("cargo:warning=Building on kernel: {}", kernel);

    let major: u32 = kernel
        .split('.')
        .next()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let minor: u32 = kernel
        .split('.')
        .nth(1)
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    if major < 5 || (major == 5 && minor < 15) {
        println!("cargo:warning=Kernel {}: WARNING — minimum supported kernel is 5.15", major);
    } else {
        println!("cargo:warning=Kernel {}: OK.", major);
    }

    println!("cargo:rerun-if-changed=build.rs");
}