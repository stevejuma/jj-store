//! Bundled upstream ACP model/tool loop, with application-owned launch policy.
mod chatgpt_token;
mod metadata;
mod provider_catalog;
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--metadata") {
        metadata::run();
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--models") {
        provider_catalog::run();
        return;
    }
    let result = match chatgpt_token::source() {
        Ok(Some(source)) => buzz_agent::run_with_local_catalog_and_token_source(source),
        Ok(None) => buzz_agent::run_with_local_catalog(),
        Err(e) => Err(e.into()),
    };
    if result.is_err() {
        eprintln!("local agent unavailable");
        std::process::exit(1);
    }
}
