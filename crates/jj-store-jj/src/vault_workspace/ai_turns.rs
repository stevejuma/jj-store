//! Durable observation boundaries, independent of conversation storage.
use super::*;
use std::collections::BTreeMap;
use std::io::Read as _;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AiTurnContext {
    pub conversation: String,
    pub turn: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub binding: Option<String>,
    #[serde(default)]
    pub session: Option<String>,
    pub agent: String,
    pub harness: String,
    pub requested_model: Option<String>,
    pub resolved_model: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiTurnOutcome {
    Completed,
    Failed,
    Cancelled,
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiTurnPhase {
    Running,
    Complete,
    Incomplete,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiTurnRecord {
    pub version: u32,
    pub context: AiTurnContext,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub phase: AiTurnPhase,
    pub outcome: Option<AiTurnOutcome>,
    pub before: Checkpoint,
    pub after: Option<Checkpoint>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiTurnPage {
    pub records: Vec<AiTurnRecord>,
    pub next: Option<String>,
}
pub(super) type ActiveTurns = BTreeMap<String, AiTurnContext>;
pub(super) fn identifier(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
    {
        return Err(Error::Binding);
    }
    Ok(())
}
impl AiTurnContext {
    pub(super) fn validate(&self) -> Result<()> {
        identifier(&self.conversation)?;
        identifier(&self.turn)?;
        for value in [
            Some(&self.agent),
            Some(&self.harness),
            self.message.as_ref(),
            self.binding.as_ref(),
            self.session.as_ref(),
            self.requested_model.as_ref(),
            self.resolved_model.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            if value.is_empty() || value.len() > 256 || value.chars().any(char::is_control) {
                return Err(Error::Binding);
            }
        }
        Ok(())
    }
    pub(super) fn key(&self) -> String {
        format!("{}/{}", self.conversation, self.turn)
    }
}
pub(super) fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(i64::MAX)
}
pub(super) fn load<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 1024 * 1024 {
        return Err(Error::Binding);
    }
    serde_json::from_slice(&bytes).map_err(jj)
}
pub(super) fn save(path: &Path, value: &impl Serialize) -> Result<()> {
    use std::io::Write as _;
    let bytes = serde_json::to_vec(value).map_err(jj)?;
    if bytes.len() > 1024 * 1024 {
        return Err(Error::Binding);
    }
    let parent = path.parent().ok_or(Error::Binding)?;
    fs::create_dir_all(parent)?;
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    file.write_all(&bytes)?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|e| Error::Io(e.error))?;
    File::open(parent)?.sync_all()?;
    Ok(())
}
impl Session {
    pub(super) fn turn_path(&self, conversation: &str, turn: &str) -> Result<PathBuf> {
        identifier(conversation)?;
        identifier(turn)?;
        Ok(self
            .binding
            .state
            .join("ai-turns")
            .join(conversation)
            .join(format!("{turn}.json")))
    }
    pub fn ai_turn(&self, conversation: &str, turn: &str) -> Result<AiTurnRecord> {
        let record: AiTurnRecord = load(&self.turn_path(conversation, turn)?)?;
        if record.version != 1
            || record.context.conversation != conversation
            || record.context.turn != turn
        {
            return Err(Error::Binding);
        }
        record.context.validate()?;
        Ok(record)
    }
    pub fn ai_turns(&self, conversation: &str, after: Option<&str>) -> Result<AiTurnPage> {
        identifier(conversation)?;
        if let Some(after) = after {
            identifier(after)?;
        }
        let directory = self.binding.state.join("ai-turns").join(conversation);
        if !directory.exists() {
            return Ok(AiTurnPage {
                records: vec![],
                next: None,
            });
        }
        let mut ids = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "json")
            {
                continue;
            }
            let Some(id) = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .map(str::to_owned)
            else {
                return Err(Error::Binding);
            };
            identifier(&id)?;
            if after.is_none_or(|after| id.as_str() > after) {
                ids.push(id);
            }
            if ids.len() > 100_000 {
                return Err(Error::Binding);
            }
        }
        ids.sort();
        let more = ids.len() > 100;
        ids.truncate(100);
        let next = more.then(|| ids.last().unwrap().clone());
        let records = ids
            .iter()
            .map(|id| self.ai_turn(conversation, id))
            .collect::<Result<_>>()?;
        Ok(AiTurnPage { records, next })
    }
    pub(super) fn active_turns(&self) -> Result<ActiveTurns> {
        let path = self.binding.state.join("ai-active.json");
        let active: ActiveTurns = if path.exists() {
            load(&path)?
        } else {
            BTreeMap::new()
        };
        for (key, context) in &active {
            context.validate()?;
            if *key != context.key() {
                return Err(Error::Binding);
            }
        }
        Ok(active)
    }
    pub(super) fn save_active(&self, active: &ActiveTurns) -> Result<()> {
        save(&self.binding.state.join("ai-active.json"), active)
    }
    /// Called once by a newly owned worker before any new captures or dispatch.
    pub fn recover_ai_turns(&self) -> Result<()> {
        for context in self.active_turns()?.values() {
            let mut record = self.ai_turn(&context.conversation, &context.turn)?;
            if record.phase == AiTurnPhase::Running {
                record.phase = AiTurnPhase::Incomplete;
                record.outcome = Some(AiTurnOutcome::Unknown);
                save(
                    &self.turn_path(&context.conversation, &context.turn)?,
                    &record,
                )?;
            }
        }
        self.save_active(&BTreeMap::new())
    }
}
