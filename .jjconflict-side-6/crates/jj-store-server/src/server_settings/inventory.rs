use super::*;
use serde_json::Value;
use std::collections::BTreeMap;
fn flatten(path: &str, value: &Value, into: &mut BTreeMap<String, String>) {
    match value {
        Value::Object(values) if !values.is_empty() => {
            for (key, value) in values {
                flatten(&format!("{path}.{key}"), value, into);
            }
        }
        Value::Array(values) if !values.is_empty() => {
            for (index, value) in values.iter().enumerate() {
                flatten(&format!("{path}[{index}]"), value, into);
            }
        }
        Value::Null => {
            into.insert(path.into(), "Not configured".into());
        }
        Value::Array(values) if values.is_empty() => {
            into.insert(path.into(), "None".into());
        }
        Value::String(value) => {
            into.insert(path.into(), value.clone());
        }
        _ => {
            into.insert(path.into(), value.to_string());
        }
    }
}
pub(super) fn values(saved: &Config, effective: &Config) -> Result<Vec<ConfiguredValue>, Error> {
    fn inventory(config: &Config) -> Result<BTreeMap<String, String>, Error> {
        let mut values = serde_json::to_value(config).map_err(|_| Error::Invalid)?;
        let table = values.as_object_mut().ok_or(Error::Invalid)?;
        for key in [
            "agent_capture",
            "search",
            "limits",
            "protection",
            "replication",
        ] {
            table.remove(key);
        }
        table
            .entry("backup")
            .or_insert_with(|| Value::Array(Vec::new()));
        let mut result = BTreeMap::new();
        for (key, value) in table {
            flatten(key, value, &mut result);
        }
        Ok(result)
    }
    let running = inventory(effective)?;
    Ok(inventory(saved)?
        .into_iter()
        .map(|(name, saved)| {
            let storage = name.starts_with("storage.") || name.starts_with("backup");
            let reason = if name.starts_with("backup") {
                "Backup execution is deferred."
            } else if storage {
                "Authority storage changes require an explicit migration."
            } else if name.starts_with("access.")
                || name == "state_dir"
                || name == "operator_policy"
                || name == "format"
            {
                "Installation identity and access are managed by setup."
            } else {
                "Managed by server or desktop launch; changes require lifecycle reconfiguration."
            };
            ConfiguredValue {
                section: if storage { "storage" } else { "service" }.into(),
                effective: running
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| "Not configured".into()),
                name,
                saved,
                reason: reason.into(),
            }
        })
        .collect())
}
