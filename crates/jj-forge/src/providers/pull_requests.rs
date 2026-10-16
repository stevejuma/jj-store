use crate::{
    client::{encode, number, text},
    *,
};
use reqwest::Method;
use serde_json::{Value, json};

pub(crate) fn decode_pr(v: &Value, provider: Provider) -> Result<PullRequest> {
    let gitlab = provider == Provider::Gitlab;
    let number = number(v, if gitlab { "iid" } else { "number" });
    if number == 0 {
        return Err(Error::Invalid);
    }
    let url = text(v, if gitlab { "web_url" } else { "html_url" });
    let parsed = url::Url::parse(&url).map_err(|_| Error::Invalid)?;
    if parsed.scheme() != "https" || !parsed.username().is_empty() || parsed.password().is_some() {
        return Err(Error::Invalid);
    }
    let state = text(v, "state");
    if !["open", "opened", "closed", "merged", "locked"].contains(&state.as_str()) {
        return Err(Error::Invalid);
    }
    let merged = v.get("merged").and_then(Value::as_bool) == Some(true)
        || v.get("merged_at").is_some_and(|x| !x.is_null())
        || text(v, "state") == "merged";
    Ok(PullRequest {
        number,
        url: text(v, if gitlab { "web_url" } else { "html_url" }),
        title: text(v, "title"),
        body: text(v, if gitlab { "description" } else { "body" }),
        head: if gitlab {
            text(v, "source_branch")
        } else {
            text(&v["head"], "ref")
        },
        sha: if gitlab {
            text(v, "sha")
        } else {
            text(&v["head"], "sha")
        },
        base: if gitlab {
            text(v, "target_branch")
        } else {
            text(&v["base"], "ref")
        },
        draft: v["draft"].as_bool().unwrap_or(false)
            || v["work_in_progress"].as_bool().unwrap_or(false),
        state: if merged {
            PrState::Merged
        } else if ["closed", "locked"].contains(&text(v, "state").as_str()) {
            PrState::Closed
        } else {
            PrState::Open
        },
        mergeable: if gitlab {
            match text(v, "detailed_merge_status").as_str() {
                "mergeable" => Some(true),
                "checking" | "unchecked" | "" => None,
                _ => Some(false),
            }
        } else {
            v["mergeable"].as_bool()
        },
        native_stack: if provider == Provider::Github {
            v["stack"]["number"].as_u64()
        } else {
            None
        },
        updated_at: text(v, "updated_at"),
    })
}
impl Client {
    pub(crate) fn pr_path(&self, n: u64) -> String {
        format!(
            "{}/{}/{n}",
            self.project(),
            if self.connection.provider == Provider::Gitlab {
                "merge_requests"
            } else {
                "pulls"
            }
        )
    }
    pub async fn pull_request(&self, n: u64) -> Result<PullRequest> {
        let v = self.request(Method::GET, &self.pr_path(n), None).await?;
        decode_pr(&v, self.connection.provider)
    }
    pub async fn list(&self) -> Result<Vec<PullRequest>> {
        let path = if self.connection.provider == Provider::Gitlab {
            "merge_requests?state=opened&scope=all"
        } else {
            "pulls?state=open"
        };
        self.matching(&format!("{}/{path}", self.project())).await
    }
    pub async fn find_by_head(&self, branch: &str) -> Result<Vec<PullRequest>> {
        if !safe_name(branch) {
            return Err(Error::Invalid);
        }
        let owner = self
            .connection
            .source_repository
            .split('/')
            .next()
            .ok_or(Error::Invalid)?;
        let path = if self.connection.provider == Provider::Gitlab {
            format!(
                "merge_requests?state=all&scope=all&source_branch={}",
                encode(branch)
            )
        } else {
            format!(
                "pulls?state=all&head={}",
                encode(&format!("{owner}:{branch}"))
            )
        };
        Ok(self
            .matching(&format!("{}/{path}", self.project()))
            .await?
            .into_iter()
            .filter(|p| p.head == branch)
            .collect())
    }
    async fn matching(&self, path: &str) -> Result<Vec<PullRequest>> {
        let values = self.pages(path).await?;
        let mut out = vec![];
        let source_id = if self.connection.provider == Provider::Gitlab {
            Some(number(
                &self
                    .request(
                        Method::GET,
                        &format!("projects/{}", encode(&self.connection.source_repository)),
                        None,
                    )
                    .await?,
                "id",
            ))
        } else {
            None
        };
        for v in values {
            let matches = if let Some(id) = source_id {
                v["source_project_id"].as_u64() == Some(id)
            } else {
                text(&v["head"]["repo"], "full_name")
                    .eq_ignore_ascii_case(&self.connection.source_repository)
            };
            if matches {
                out.push(decode_pr(&v, self.connection.provider)?);
            }
        }
        Ok(out)
    }
    pub async fn create(&self, s: &Segment, draft: bool) -> Result<PullRequest> {
        let gl = self.connection.provider == Provider::Gitlab;
        let path = format!(
            "{}/{}",
            self.project(),
            if gl { "merge_requests" } else { "pulls" }
        );
        let payload = if gl {
            let source = self
                .request(
                    Method::GET,
                    &format!("projects/{}", encode(&self.connection.source_repository)),
                    None,
                )
                .await?;
            let target = self.request(Method::GET, &self.project(), None).await?;
            json!({"source_project_id":number(&source,"id"),"target_project_id":number(&target,"id"),"source_branch":s.branch,"target_branch":s.base,"title":if draft{format!("Draft: {}",s.title)}else{s.title.clone()},"description":s.body})
        } else {
            let owner = self
                .connection
                .source_repository
                .split('/')
                .next()
                .ok_or(Error::Invalid)?;
            json!({"head":format!("{owner}:{}",s.branch),"head_repo":self.connection.source_repository.rsplit('/').next().ok_or(Error::Invalid)?,"base":s.base,"title":if self.connection.provider==Provider::Forgejo && draft {format!("WIP: {}",s.title)}else{s.title.clone()},"body":s.body,"draft":draft})
        };
        let path = if gl && self.connection.fork() {
            format!(
                "projects/{}/merge_requests",
                encode(&self.connection.source_repository)
            )
        } else {
            path
        };
        let v = self.request(Method::POST, &path, Some(payload)).await?;
        decode_pr(&v, self.connection.provider).map_err(|_| Error::Unknown)
    }
    pub async fn update(
        &self,
        n: u64,
        title: Option<&str>,
        body: Option<&str>,
        base: Option<&str>,
    ) -> Result<PullRequest> {
        let gl = self.connection.provider == Provider::Gitlab;
        let mut p = serde_json::Map::new();
        for (key, v) in [
            ("title", title),
            (if gl { "description" } else { "body" }, body),
            (if gl { "target_branch" } else { "base" }, base),
        ] {
            if let Some(v) = v {
                p.insert(key.into(), v.into());
            }
        }
        let v = self
            .request(
                if gl { Method::PUT } else { Method::PATCH },
                &self.pr_path(n),
                Some(Value::Object(p)),
            )
            .await?;
        decode_pr(&v, self.connection.provider).map_err(|_| Error::Unknown)
    }
    pub async fn draft(&self, n: u64, draft: bool) -> Result<PullRequest> {
        match self.connection.provider {
            Provider::Forgejo => {
                let pr = self.pull_request(n).await?;
                let title = pr.title.strip_prefix("WIP: ").unwrap_or(&pr.title);
                let title = if draft {
                    format!("WIP: {title}")
                } else {
                    title.to_string()
                };
                let result = self.update(n, Some(&title), None, None).await?;
                if result.draft != draft {
                    return Err(Error::Unsupported);
                }
                Ok(result)
            }
            Provider::Gitlab => {
                let pr = self.pull_request(n).await?;
                let title = pr.title.strip_prefix("Draft: ").unwrap_or(&pr.title);
                self.update(
                    n,
                    Some(&if draft {
                        format!("Draft: {title}")
                    } else {
                        title.to_string()
                    }),
                    None,
                    None,
                )
                .await
            }
            Provider::Github => {
                let v = self.request(Method::GET, &self.pr_path(n), None).await?;
                let mutation = if draft {
                    "convertPullRequestToDraft"
                } else {
                    "markPullRequestReadyForReview"
                };
                let result=self.request(Method::POST,"graphql",Some(json!({"query":format!("mutation($id:ID!){{{mutation}(input:{{pullRequestId:$id}}){{pullRequest{{id}}}}}}"),"variables":{"id":text(&v,"node_id")}}))).await?;
                if result.get("errors").is_some() {
                    return Err(Error::Unknown);
                }
                self.pull_request(n).await.map_err(|_| Error::Unknown)
            }
        }
    }
    pub async fn unstack(&self, pr: &PullRequest) -> Result<()> {
        let Some(stack) = pr.native_stack else {
            return Ok(());
        };
        self.request(
            Method::POST,
            &format!("{}/stacks/{stack}/unstack", self.project()),
            Some(json!({"pull_requests":[pr.number]})),
        )
        .await?;
        Ok(())
    }
    /// Membership is ordered and exact; a larger stack is not this submitted chain.
    pub async fn native_stack_matches(&self, prs: &[PullRequest]) -> Result<bool> {
        let Some(stack) = prs.first().and_then(|p| p.native_stack) else {
            return Ok(false);
        };
        if prs.iter().any(|p| p.native_stack != Some(stack)) {
            return Ok(false);
        }
        let stacks = self.pages(&format!("{}/stacks", self.project())).await?;
        let wanted: Vec<_> = prs.iter().map(|p| p.number).collect();
        Ok(stacks.iter().any(|s| {
            number(s, "number") == stack
                && s["pull_requests"].as_array().is_some_and(|a| {
                    a.iter().map(|p| number(p, "number")).collect::<Vec<_>>() == wanted
                })
        }))
    }
    pub async fn native_stack(&self, prs: &[PullRequest]) -> Result<()> {
        if !self.capabilities().native_stacks || prs.len() < 2 {
            return Ok(());
        }
        if prs.iter().any(|p| p.native_stack.is_some()) {
            if self.native_stack_matches(prs).await? {
                return Ok(());
            }
            // Reshaping is an explicit separate intent; never dissolve unowned membership.
            return Err(Error::Stale);
        }
        self.request(
            Method::POST,
            &format!("{}/stacks", self.project()),
            Some(json!({"pull_requests":prs.iter().map(|p|p.number).collect::<Vec<_>>()})),
        )
        .await?;
        Ok(())
    }
}
