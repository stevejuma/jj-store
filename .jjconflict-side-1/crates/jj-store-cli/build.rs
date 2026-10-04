use std::process::Command;
fn main() {
    println!("cargo:rerun-if-env-changed=JJ_STORE_BUILD_REVISION");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    let revision = std::env::var("JJ_STORE_BUILD_REVISION")
        .ok()
        .or_else(|| {
            let result = Command::new("jj")
                .args([
                    "--ignore-working-copy",
                    "--no-pager",
                    "log",
                    "-r",
                    "@",
                    "--no-graph",
                    "-T",
                    "commit_id",
                ])
                .output()
                .ok()?;
            result
                .status
                .success()
                .then(|| String::from_utf8_lossy(&result.stdout).trim().to_owned())
        })
        .filter(|value| {
            !value.is_empty() && value.len() <= 64 && value.bytes().all(|c| c.is_ascii_hexdigit())
        })
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=JJ_STORE_REVISION={revision}");
    println!(
        "cargo:rustc-env=JJ_STORE_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-env=JJ_STORE_PROFILE={}",
        std::env::var("PROFILE").unwrap()
    );
}
