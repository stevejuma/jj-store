//! Bundled upstream ACP model/tool loop, with application-owned launch policy.
fn main() {
    if buzz_agent::run_with_local_catalog().is_err() {
        eprintln!("local agent unavailable");
        std::process::exit(1);
    }
}
