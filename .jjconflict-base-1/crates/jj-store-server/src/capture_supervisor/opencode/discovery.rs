use super::*;
use std::{collections::VecDeque, time::Instant};
pub(super) struct ApiDiscoveryWork {
    api: OpenCodeReadApi,
    resources: CodexSupervisorResources,
    pending: VecDeque<String>,
    projects: VecDeque<String>,
    directories: VecDeque<String>,
    seen_directories: BTreeSet<String>,
    offered: BTreeSet<String>,
    interval: Duration,
    next: Instant,
    unsupported: bool,
}
pub(super) struct ApiDiscoveryOutput {
    pub candidate: Option<String>,
    pub unavailable: bool,
}
impl ApiDiscoveryWork {
    pub fn new(
        api: OpenCodeReadApi,
        resources: CodexSupervisorResources,
        interval: Duration,
    ) -> Self {
        Self {
            api,
            resources,
            pending: VecDeque::new(),
            projects: VecDeque::new(),
            directories: VecDeque::new(),
            seen_directories: BTreeSet::new(),
            offered: BTreeSet::new(),
            interval,
            next: Instant::now(),
            unsupported: false,
        }
    }
    pub fn rescan(&mut self) {
        self.next = Instant::now();
    }
    async fn advance(&mut self) -> Result<Option<String>> {
        if let Some(candidate) = self.pending.pop_front() {
            return Ok(Some(candidate));
        }
        if let Some(directory) = self.directories.pop_front() {
            let proof = match self.resources.admission.inspect(&directory).await {
                Ok(proof) => proof,
                Err(
                    crate::capture_admission::Error::Unbound
                    | crate::capture_admission::Error::Limit,
                ) => return Ok(None),
                Err(_) => return Err(Error::Service("OpenCode discovery admission unavailable")),
            };
            self.resources
                .authority
                .bind(&self.resources.host, &self.resources.admission, &proof)
                .await?;
            let discovery = self.api.discover_active_in(&directory).await?;
            self.resources
                .authority
                .bind(&self.resources.host, &self.resources.admission, &proof)
                .await?;
            self.unsupported |= discovery.unsupported > 0;
            for candidate in discovery.candidates {
                let id = candidate.native_session_key().to_owned();
                if !self.offered.contains(&id) && self.offered.len() == 4096 {
                    return Err(Error::Service(
                        "OpenCode discovery session budget exhausted",
                    ));
                }
                if self.offered.insert(id.clone()) {
                    self.pending.push_back(id);
                }
            }
            return Ok(self.pending.pop_front());
        }
        if let Some(project) = self.projects.pop_front() {
            let inventory = if project == "global" {
                self.api.global_session_directories().await?
            } else {
                self.api.project_directories(&project).await?
            };
            self.unsupported |= inventory.unsupported > 0;
            for directory in inventory.directories {
                if !self.seen_directories.contains(&directory) {
                    if self.seen_directories.len() == 4096 {
                        return Err(Error::Service(
                            "OpenCode discovery directory budget exhausted",
                        ));
                    }
                    self.seen_directories.insert(directory.clone());
                    self.directories.push_back(directory);
                }
            }
            return Ok(None);
        }
        if Instant::now() < self.next {
            return Ok(None);
        }
        self.next = Instant::now() + self.interval;
        self.seen_directories.clear();
        self.offered.clear();
        // Recovery candidates survive a later new-directory enumeration failure.
        for id in self
            .resources
            .sources
            .admitted_opencode_sessions(self.api.endpoint_digest())
            .map_err(|_| Error::Service("OpenCode recovery index unavailable"))?
        {
            if self.offered.insert(id.clone()) {
                self.pending.push_back(id);
            }
        }
        let inventory = self.api.project_inventory().await?;
        self.unsupported |= inventory.unsupported > 0;
        self.projects = inventory.projects.into();
        Ok(self.pending.pop_front())
    }
}
#[async_trait::async_trait]
impl OwnedWork for ApiDiscoveryWork {
    type Command = ();
    type Output = ApiDiscoveryOutput;
    async fn step(&mut self, _: ()) -> Self::Output {
        let result = self.advance().await;
        let unsupported = std::mem::take(&mut self.unsupported);
        ApiDiscoveryOutput {
            unavailable: result.is_err() || unsupported,
            candidate: result.ok().flatten(),
        }
    }
}
