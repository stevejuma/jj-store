use crate::{
    client::{encode, text},
    *,
};
use reqwest::Method;
use serde_json::json;
impl Client {
    pub async fn merge(&self, pr: &PullRequest, method: &str) -> Result<MergeResult> {
        if !["merge", "squash", "rebase"].contains(&method)
            || pr.draft
            || pr.state != PrState::Open
            || pr.mergeable != Some(true)
        {
            return Err(Error::Stale);
        }
        let checks = self.checks(pr).await?;
        if !checks.complete
            || checks
                .checks
                .iter()
                .any(|c| !matches!(c.state, CheckState::Passed | CheckState::Skipped))
        {
            return Err(Error::Stale);
        }
        let (path, payload) = match self.connection.provider {
            Provider::Github => (
                format!("{}/merge-async", self.pr_path(pr.number)),
                json!({"sha":pr.sha,"merge_method":method,"merge_action":"default","bypass_rules":false}),
            ),
            Provider::Gitlab => (
                format!("{}/merge", self.pr_path(pr.number)),
                json!({"sha":pr.sha,"squash":method=="squash","should_remove_source_branch":false}),
            ),
            Provider::Forgejo => (
                format!("{}/merge", self.pr_path(pr.number)),
                json!({"Do":method,"head_commit_id":pr.sha,"force_merge":false,"delete_branch_after_merge":false}),
            ),
        };
        if self.connection.provider == Provider::Gitlab && method == "rebase" {
            return Err(Error::Unsupported);
        }
        let v = self
            .request(
                if self.connection.provider == Provider::Forgejo {
                    Method::POST
                } else {
                    Method::PUT
                },
                &path,
                Some(payload),
            )
            .await?;
        let state = if self.connection.provider == Provider::Github {
            text(&v, "status")
        } else if self
            .pull_request(pr.number)
            .await
            .map_err(|_| Error::Unknown)?
            .state
            == PrState::Merged
        {
            "merged".into()
        } else {
            "pending".into()
        };
        Ok(MergeResult {
            state,
            receipt: v["details"]["uuid"].as_str().map(str::to_string),
            sha: v["details"]["sha"]
                .as_str()
                .or(v["sha"].as_str())
                .map(str::to_string),
        })
    }
    pub async fn merge_status(&self, n: u64, receipt: &str) -> Result<MergeResult> {
        if self.connection.provider != Provider::Github {
            return Err(Error::Unsupported);
        }
        if !safe_name(receipt) {
            return Err(Error::Invalid);
        }
        let v = self
            .request(
                Method::GET,
                &format!("{}/merge-async/{}", self.pr_path(n), encode(receipt)),
                None,
            )
            .await?;
        Ok(MergeResult {
            state: text(&v, "status"),
            receipt: Some(receipt.into()),
            sha: v["details"]["sha"].as_str().map(str::to_string),
        })
    }
}
