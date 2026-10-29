//! The shared native verifier owns helper inventory, hashes, target and ABI checks.
use crate::Result;
use lapis_ai_runtime::{Config, ResourceBundle};
use std::path::Path;

pub(crate) fn configure(root: &Path) -> Result<Config> {
    let root = root.join("lapis-ai");
    let bundle = ResourceBundle::load(&root).map_err(|error| error.to_string())?;
    let mut config = Config::parse("version=1", &root).map_err(|error| error.to_string())?;
    bundle.configure(&mut config);
    Ok(config)
}
