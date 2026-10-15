//! Owned, serializable contracts. No upstream JJ or provider SDK types escape.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "ForgeProvider")]
pub enum Provider {
    Github,
    Gitlab,
    Forgejo,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ForgeConnection")]
pub struct Connection {
    pub provider: Provider,
    /// Approved API root, e.g. https://api.github.com or https://forge.example/api/v1/.
    pub api_url: String,
    /// Slash-separated project path; GitLab subgroups are preserved.
    pub repository: String,
    pub source_repository: String,
    pub remote: String,
    #[serde(default = "default_remote")]
    pub target_remote: String,
    pub target: String,
    pub native_stacks: bool,
}
fn default_remote() -> String {
    "origin".into()
}
impl Connection {
    pub fn validate(&self) -> crate::Result<()> {
        let u = url::Url::parse(&self.api_url).map_err(|_| crate::Error::Invalid)?;
        if u.scheme() != "https"
            || u.host_str().is_none()
            || !u.username().is_empty()
            || u.password().is_some()
            || u.query().is_some()
            || u.fragment().is_some()
            || [&self.repository, &self.source_repository]
                .iter()
                .any(|v| v.split('/').count() < 2 || v.split('/').any(|s| !safe_name(s)))
            || !safe_name(&self.remote)
            || !safe_name(&self.target_remote)
            || !safe_name(&self.target)
        {
            return Err(crate::Error::Invalid);
        }
        Ok(())
    }
    pub fn fork(&self) -> bool {
        self.repository != self.source_repository
    }
}
pub fn safe_name(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 1024
        && !v.starts_with('-')
        && !v.contains("..")
        && !v.starts_with('/')
        && !v.ends_with('/')
        && !v.contains("//")
        && v.chars()
            .all(|c| c.is_ascii_alphanumeric() || "._/-".contains(c))
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeCapabilities")]
pub struct Capabilities {
    pub edit: bool,
    pub draft: bool,
    pub merge: bool,
    pub auto_merge: bool,
    pub checks: bool,
    pub logs: bool,
    pub rerun: bool,
    pub rerun_all: bool,
    pub cancel: bool,
    pub merge_methods: Vec<String>,
    pub native_stacks: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeSegment")]
pub struct Segment {
    pub branch: String,
    pub head: String,
    pub base: String,
    pub title: String,
    pub body: String,
    pub remote_head: Option<String>,
    pub shared: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "ForgeStepKind")]
pub enum StepKind {
    Unstack,
    Push,
    Create,
    Retarget,
    Description,
    NativeStack,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "ForgePrState")]
pub enum PrState {
    Open,
    Closed,
    Merged,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgePullRequest")]
pub struct PullRequest {
    pub number: u64,
    pub url: String,
    pub title: String,
    pub body: String,
    pub head: String,
    pub sha: String,
    pub base: String,
    pub draft: bool,
    pub state: PrState,
    pub mergeable: Option<bool>,
    pub native_stack: Option<u64>,
    pub updated_at: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgePlanItem")]
pub struct PlanItem {
    pub segment: Segment,
    pub pr: Option<PullRequest>,
    pub steps: Vec<StepKind>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgePlan")]
pub struct Plan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    pub request_id: String,
    pub operation: String,
    pub connection: Connection,
    pub draft: bool,
    pub cumulative: bool,
    pub items: Vec<PlanItem>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "ForgeOutcome")]
pub enum Outcome {
    Pending,
    Running,
    Complete,
    Rejected,
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeStep")]
pub struct Step {
    #[serde(default)]
    pub fingerprint: Option<String>,
    pub branch: String,
    pub kind: StepKind,
    pub outcome: Outcome,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeReceipt")]
pub struct Receipt {
    pub plan: Plan,
    pub steps: Vec<Step>,
    pub outcome: Outcome,
    pub pull_requests: Vec<PullRequest>,
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "ForgeCheckState")]
pub enum CheckState {
    Queued,
    Running,
    Passed,
    Failed,
    Cancelled,
    Skipped,
    Manual,
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeCheck")]
pub struct Check {
    pub id: String,
    pub name: String,
    pub state: CheckState,
    pub sha: String,
    pub url: Option<String>,
    pub run_id: Option<String>,
    pub log_id: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeChecks")]
pub struct Checks {
    pub sha: String,
    pub checks: Vec<Check>,
    pub complete: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeMergeResult")]
pub struct MergeResult {
    pub state: String,
    pub receipt: Option<String>,
    pub sha: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "ForgePrAction")]
pub enum PrAction {
    Edit {
        title: String,
        body: String,
        expected_body: String,
    },
    Draft {
        draft: bool,
    },
    Merge {
        method: String,
        expected_sha: String,
    },
    MergeStatus {
        receipt: String,
    },
    Checks,
    Jobs {
        run: String,
    },
    Logs {
        run: String,
        job: String,
    },
    Rerun {
        run: String,
        failed_only: bool,
    },
    Cancel {
        run: String,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "ForgeAction")]
pub enum Action {
    Detect {
        auto_connect: bool,
    },
    BindAccount {
        account: String,
        connection: Connection,
    },
    Status,
    RestackPreview {
        bookmark: String,
    },
    Restack {
        plan: RestackPlan,
    },
    Configure {
        connection: Connection,
    },
    Preview {
        bookmark: String,
        draft: bool,
    },
    Submit {
        plan: Plan,
    },
    Recover {
        request_id: String,
    },
    OperationStatus {
        request_id: String,
    },
    PullRequest {
        number: u64,
    },
    Pr {
        request_id: String,
        number: u64,
        action: PrAction,
    },
}
/// The token is intentionally not Debug and must never enter a journal.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ForgeRequest")]
pub struct Request {
    pub version: u32,
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    pub action: Action,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeResponse")]
pub struct Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detection: Option<crate::accounts::Detection>,
    pub notice: Option<String>,
    pub connection: Option<Connection>,
    pub capabilities: Capabilities,
    pub restack: Option<RestackReceipt>,
    pub pull_requests: Vec<PullRequest>,
    pub plan: Option<Plan>,
    pub receipt: Option<Receipt>,
    pub checks: Option<Checks>,
    pub log: Option<String>,
    pub merge: Option<MergeResult>,
    pub merges: std::collections::BTreeMap<u64, MergeResult>,
    pub operation: Option<crate::journal::Operation>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeRestackPlan")]
pub struct RestackPlan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    pub request_id: String,
    pub operation: String,
    pub revision: u64,
    pub connection: Connection,
    pub bookmark: String,
    pub branch: String,
    pub expected_base: String,
    pub target_head: String,
    pub merged: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeRestackReceipt")]
pub struct RestackReceipt {
    pub plan: RestackPlan,
    pub outcome: Outcome,
    pub operation: Option<String>,
    pub submission: Option<Plan>,
}
