// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.



fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}

// build.rs

use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

fn main() {
    // 获取当前的 UNIX 时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 设置环境变量 TEST_FOO
    println!("cargo:rerun-if-changed=build.rs"); // 使得每次 build.rs 改变时重新运行
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 检查环境变量并设置特性
    if let Ok(_) = env::var("ENABLE_PASS") {
        println!("cargo:rustc-cfg=pass");
    }
}
