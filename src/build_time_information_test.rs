shadow!(build);

pub fn build_time_information_tests () {
    println!("is_debug: {}", shadow_rs::is_debug());      // check if this is a debug build

    println!("build_version: {}", build::version());           // the version (description binary detail information)
    println!("build::BRANCH: {}", build::BRANCH);              // the branch, e.g. 'master'
    println!("build::TAG: {}", build::TAG);                 // the tag, e.g. 'v1.0.0'
    println!("build::SHORT_COMMIT: {}", build::SHORT_COMMIT);        // short commit hash, e.g. '8405e28e'
    println!("build::COMMIT_HASH: {}", build::COMMIT_HASH);         // full commit hash, e.g. '8405e28e64080a09525a6cf1b07c22fcaf71a5c5'
    println!("build::COMMIT_DATE: {}", build::COMMIT_DATE);         // commit date, e.g. '2020-08-16 11:52:47'
    println!("build::COMMIT_AUTHOR: {}", build::COMMIT_AUTHOR);       // commit author, e.g. 'baoyachi'
    println!("build::COMMIT_EMAIL: {}", build::COMMIT_EMAIL);        // commit email, e.g. 'example@gmail.com'

    println!("build::BUILD_OS: {}", build::BUILD_OS);            // the OS that built the binary, e.g. 'macos-x86_64'
    println!("build::RUST_VERSION: {}", build::RUST_VERSION);        // rustc version e.g. 'rustc 1.45.0 (5c1f21c3b 2020-07-13)'
    println!("build::RUST_CHANNEL: {}", build::RUST_CHANNEL);        // rust toolchain e.g. 'stable-x86_64-apple-darwin (default)'
    println!("build::CARGO_VERSION: {}", build::CARGO_VERSION);       // cargo version e.g. 'cargo 1.45.0 (744bd1fbb 2020-06-15)'
    println!("build::PKG_VERSION: {}", build::PKG_VERSION);         // e.g. '0.3.13'
    println!("build::CARGO_TREE: {}", build::CARGO_TREE);          // e.g. the output of '$ cargo tree'

    println!("build::PROJECT_NAME: {}", build::PROJECT_NAME);        // your project name, e.g. 'shadow-rs'
    println!("build::BUILD_TIME: {}", build::BUILD_TIME);          // time when start build occurred, e.g. '2020-08-16 14:50:25'
    println!("build::BUILD_RUST_CHANNEL: {}", build::BUILD_RUST_CHANNEL);  // e.g. 'debug'
}
