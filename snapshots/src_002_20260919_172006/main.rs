// main.rs
// Date: Thu Sep 19 2026
// Educational and Practice Rust Programming Language Code

// Project: Ownership and memory layout
// Goal: ...
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

    let s1 = size_of::<String>();
    let s2 = size_of::<&String>();
    let s3 = size_of::<&'static String>();
    let st1 = size_of::<&str>();
    let st2 = size_of::<&'static str>();

    println!("size of string is: {}", s1);
    println!("size of &string is: {}", s2);
    println!("size of &'static string is: {}", s3);
    println!("size of &str is: {}", st1);
    println!("size of &'static is: {}", st2);

    println!("\nThe End ...\n");
}
