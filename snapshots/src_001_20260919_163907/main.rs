// main.rs
// Date: Thu Sep 19 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning Chapter 15
// Goal: Using Smart Pointer: Treating smart pointer like regular reference
// Dependency: Without dependency

// rustc 1.100.0-nightly (420ed2a0c 2026-09-18)
// binary: rustc
// commit-hash: 420ed2a0c3d7225b1744266fd884d431b4d8cfe0
// commit-date: 2026-09-18
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.1

// cargo 1.100.0-nightly (495c385d0 2026-09-16)
// release: 1.100.0-nightly
// commit-hash: 495c385d0875c4ba51eb72ea0448a2d4c018b8d4
// commit-date: 2026-09-16
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.5-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

fn main() {
    println!("\n");

    let mut base_name: String = String::from("Hinata");
    let my_name: &mut String = &mut base_name;

    my_name.push_str(" Chikao");

    // As long as my_name is in use, base_name is not available.
    println!("value of my_name is: {}", my_name);

    // After last using of my_name base_name variable becomes usable.
    println!("value of name is: {}", base_name);

    println!("\nThe End ...\n");
}
