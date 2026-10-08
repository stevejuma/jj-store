//! Bundled upstream ACP model/tool loop, with application-owned launch policy.
mod provider_catalog;
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--models") {
        provider_catalog::run();
        return;
    }
    if buzz_agent::run_with_local_catalog().is_err() {
        eprintln!("local agent unavailable");
        std::process::exit(1);
    }
}
