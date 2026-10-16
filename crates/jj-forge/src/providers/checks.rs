use crate::{
    client::{encode, number, text},
    *,
};
use reqwest::Method;
pub(super) fn web_url(value: String) -> Option<String> {
    url::Url::parse(&value)
        .ok()
        .filter(|u| u.scheme() == "https" && u.username().is_empty() && u.password().is_none())
        .map(|_| value)
}

pub(super) fn timestamp(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty() && value.len() <= 64)
        .map(str::to_owned)
}

pub fn check_state(status: &str, conclusion: &str) -> CheckState {
    match if conclusion.is_empty() {
        status
    } else {
        conclusion
    } {
        "success" | "successful" => CheckState::Passed,
        "failure" | "failed" | "timed_out" | "action_required" | "startup_failure" | "error" => {
            CheckState::Failed
        }
        "cancelled" | "canceled" => CheckState::Cancelled,
        "skipped" | "neutral" => CheckState::Skipped,
        "in_progress" | "running" => CheckState::Running,
        "pending" | "queued" | "requested" | "waiting" | "created" | "preparing" | "scheduled" => {
            CheckState::Queued
        }
        "manual" | "blocked" => CheckState::Manual,
        _ => CheckState::Unknown,
    }
}
impl Client {
    pub async fn checks(&self, pr: &PullRequest) -> Result<Checks> {
        let project = self.project();
        let (path, kind) = match self.connection.provider {
            Provider::Github => return self.github_checks(pr).await,
            Provider::Forgejo => (format!("{project}/actions/runs"), "run"),
            Provider::Gitlab => (format!("{}/pipelines", self.pr_path(pr.number)), "pipeline"),
        };
        let values = self.pages(&path).await?;
        let mut checks = vec![];
        for v in values {
            let sha = text(
                &v,
                if kind == "pipeline" {
                    "sha"
                } else {
                    "head_sha"
                },
            );
            if sha != pr.sha {
                continue;
            }
            let id = number(&v, "id").to_string();
            checks.push(Check {
                id: id.clone(),
                name: if text(&v, "name").is_empty() {
                    format!("{kind} {id}")
                } else {
                    text(&v, "name")
                },
                state: check_state(&text(&v, "status"), &text(&v, "conclusion")),
                sha,
                url: web_url(text(
                    &v,
                    if kind == "pipeline" {
                        "web_url"
                    } else {
                        "html_url"
                    },
                )),
                run_id: Some(id),
                log_id: None,
                started_at: if kind == "pipeline" {
                    timestamp(&v, "started_at")
                } else {
                    timestamp(&v, "run_started_at")
                },
                completed_at: if kind == "pipeline" {
                    timestamp(&v, "finished_at")
                } else {
                    None
                },
            });
        }
        if self.connection.provider != Provider::Gitlab {
            let status_path = if self.connection.provider == Provider::Github {
                format!("{project}/commits/{}/statuses", encode(&pr.sha))
            } else {
                format!("{project}/statuses/{}", encode(&pr.sha))
            };
            let values = self.pages(&status_path).await?;
            let mut contexts = std::collections::BTreeSet::new();
            for v in values {
                let name = text(&v, "context");
                if !contexts.insert(name.clone()) {
                    continue;
                }
                checks.push(Check {
                    id: format!("status:{}", number(&v, "id")),
                    name,
                    state: check_state(&text(&v, "state"), ""),
                    sha: pr.sha.clone(),
                    url: web_url(text(&v, "target_url")),
                    run_id: None,
                    log_id: None,
                    started_at: None,
                    completed_at: None,
                });
            }
        }
        Ok(Checks {
            sha: pr.sha.clone(),
            checks,
            complete: true,
        })
    }
    pub async fn verify_run(&self, pr: &PullRequest, run: &str) -> Result<()> {
        numeric(run)?;
        let path = if self.connection.provider == Provider::Gitlab {
            format!("{}/pipelines/{run}", self.project())
        } else {
            format!("{}/actions/runs/{run}", self.project())
        };
        let v = self.request(Method::GET, &path, None).await?;
        let sha = text(
            &v,
            if self.connection.provider == Provider::Gitlab {
                "sha"
            } else {
                "head_sha"
            },
        );
        if sha != pr.sha {
            return Err(Error::Stale);
        }
        Ok(())
    }
    pub async fn jobs(&self, pr: &PullRequest, run: &str) -> Result<Checks> {
        self.verify_run(pr, run).await?;
        let path = match self.connection.provider {
            Provider::Gitlab => format!("{}/pipelines/{run}/jobs", self.project()),
            _ => format!("{}/actions/runs/{run}/jobs", self.project()),
        };
        let mut checks = vec![];
        for v in self.pages(&path).await? {
            let id = number(&v, "id").to_string();
            checks.push(Check {
                id: id.clone(),
                name: text(&v, "name"),
                state: check_state(&text(&v, "status"), &text(&v, "conclusion")),
                sha: pr.sha.clone(),
                url: web_url(text(
                    &v,
                    if self.connection.provider == Provider::Gitlab {
                        "web_url"
                    } else {
                        "html_url"
                    },
                )),
                run_id: Some(run.into()),
                log_id: Some(id),
                started_at: timestamp(&v, "started_at"),
                completed_at: timestamp(
                    &v,
                    if self.connection.provider == Provider::Gitlab {
                        "finished_at"
                    } else {
                        "completed_at"
                    },
                ),
            });
        }
        Ok(Checks {
            sha: pr.sha.clone(),
            checks,
            complete: true,
        })
    }
    pub async fn logs(&self, job: &str) -> Result<String> {
        numeric(job)?;
        let path = match self.connection.provider {
            Provider::Gitlab => format!("{}/jobs/{job}/trace", self.project()),
            _ => format!("{}/actions/jobs/{job}/logs", self.project()),
        };
        let data = self.bytes(Method::GET, &path, None).await?;
        String::from_utf8(data).map_err(|_| Error::Unsupported)
    }
    pub async fn control_run(&self, run: &str, cancel: bool, failed_only: bool) -> Result<()> {
        numeric(run)?;
        if !cancel && !failed_only && !self.capabilities().rerun_all {
            return Err(Error::Unsupported);
        }
        if !cancel && !self.capabilities().rerun {
            return Err(Error::Unsupported);
        }
        let path = match self.connection.provider {
            Provider::Gitlab => format!(
                "{}/pipelines/{run}/{}",
                self.project(),
                if cancel { "cancel" } else { "retry" }
            ),
            _ => format!(
                "{}/actions/runs/{run}/{}",
                self.project(),
                if cancel {
                    "cancel"
                } else if failed_only {
                    "rerun-failed-jobs"
                } else {
                    "rerun"
                }
            ),
        };
        self.request(Method::POST, &path, None).await?;
        Ok(())
    }
}
pub(crate) fn numeric(v: &str) -> Result<()> {
    if v.is_empty() || v.len() > 24 || !v.chars().all(|c| c.is_ascii_digit()) {
        Err(Error::Invalid)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timing_only_uses_explicit_provider_fields() {
        let v = serde_json::json!({"started_at":"2026-10-15T12:00:00Z", "completed_at":"2026-10-15T12:00:05Z", "updated_at":"2026-10-15T13:00:00Z"});
        assert_eq!(
            timestamp(&v, "started_at").as_deref(),
            Some("2026-10-15T12:00:00Z")
        );
        assert_eq!(
            timestamp(&v, "completed_at").as_deref(),
            Some("2026-10-15T12:00:05Z")
        );
        assert_eq!(timestamp(&v, "finished_at"), None);
        assert_eq!(
            timestamp(&serde_json::json!({"completed_at":null}), "completed_at"),
            None
        );
        assert_eq!(
            timestamp(
                &serde_json::json!({"completed_at":"x".repeat(65)}),
                "completed_at"
            ),
            None
        );
    }
    #[test]
    fn provider_states_do_not_promote_pending_manual_or_unknown_to_passed() {
        assert_eq!(check_state("in_progress", ""), CheckState::Running);
        assert_eq!(check_state("completed", "failure"), CheckState::Failed);
        assert_eq!(check_state("manual", ""), CheckState::Manual);
        assert_eq!(
            check_state("completed", "new-provider-conclusion"),
            CheckState::Unknown
        );
        assert_eq!(check_state("success", ""), CheckState::Passed);
        assert!(web_url("javascript:alert(1)".into()).is_none());
    }
}
