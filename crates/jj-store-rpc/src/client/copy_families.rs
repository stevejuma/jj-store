use super::*;
use crate::copy_families;
use jj_store_service::transport::{CopyFamily, CopyFamilyLimits};

impl NativeClient {
    /// Complete identity-only family. Optional seeds may be client-local staged
    /// ancestors; required seeds must be committed in the server selection.
    pub async fn copy_family(
        &self,
        seeds: &BTreeSet<m::CopyId>,
        required: &BTreeSet<m::CopyId>,
        limits: CopyFamilyLimits,
        deadline: Instant,
    ) -> Result<CopyFamily, Status> {
        copy_families::admit(seeds.len(), limits)?;
        if !required.is_subset(seeds) {
            return Err(Status::invalid_argument("required copy seeds"));
        }
        let deadline = deadline.min(
            Instant::now()
                .checked_add(self.options.timeout)
                .ok_or_else(|| Status::deadline_exceeded("copy family deadline"))?,
        );
        let result = tokio::time::timeout_at(deadline, async {
            let timeout_millis = deadline
                .saturating_duration_since(Instant::now())
                .as_millis()
                .min(3_600_000) as u32;
            if timeout_millis == 0 {
                return Err(Status::deadline_exceeded("copy family deadline"));
            }
            let body = wire::CopyFamilyRequest {
                repository: self.repository.as_bytes().to_vec(),
                seeds: seeds
                    .iter()
                    .map(|id| wire::CopyFamilySeed {
                        id: id.as_bytes().to_vec(),
                        required: required.contains(id),
                    })
                    .collect(),
                max_nodes: limits.max_nodes.min(MAX_IDS) as u32,
                max_work: limits.max_work as u64,
                max_bytes: limits.max_bytes as u64,
                timeout_millis,
            };
            if Instant::now() >= deadline {
                return Err(Status::deadline_exceeded("copy family deadline"));
            }
            let mut request = self.request(body);
            request.set_timeout(deadline.saturating_duration_since(Instant::now()));
            let reply = self
                .wire()
                .copy_family(request)
                .await
                .map_err(|error| deadline_status(error, deadline))?
                .into_inner();
            copy_families::decode(reply, seeds, required, limits)
        })
        .await
        .map_err(|_| Status::deadline_exceeded("copy family deadline"))??;
        if Instant::now() >= deadline {
            return Err(Status::deadline_exceeded("copy family deadline"));
        }
        Ok(result)
    }
}

// Tonic maps its local TimeoutExpired into Cancelled. Recognize that concrete
// source rather than treating a remote Cancelled status as a timeout.
fn deadline_status(error: Status, deadline: Instant) -> Status {
    let mut source = std::error::Error::source(&error);
    while let Some(current) = source {
        if current.is::<tonic::TimeoutExpired>() {
            return Status::deadline_exceeded("copy family deadline");
        }
        source = current.source();
    }
    if Instant::now() >= deadline {
        Status::deadline_exceeded("copy family deadline")
    } else {
        error
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_transport_timeout_retains_deadline_semantics() {
        let deadline = Instant::now() + Duration::from_secs(10);
        let local = Status::from_error(Box::new(tonic::TimeoutExpired(())));
        assert_eq!(local.code(), Code::Cancelled);
        assert_eq!(
            deadline_status(local, deadline).code(),
            Code::DeadlineExceeded
        );
        assert_eq!(
            deadline_status(Status::cancelled("remote canceled"), deadline).code(),
            Code::Cancelled
        );
        assert_eq!(
            deadline_status(Status::unimplemented("older peer"), deadline).code(),
            Code::Unimplemented
        );
        assert_eq!(
            deadline_status(Status::unimplemented("late peer"), Instant::now()).code(),
            Code::DeadlineExceeded
        );
    }
}
