//! A separate metadata-home checkout, never a code attachment or change summary.
use super::*;
impl Supervisor {
    pub(super) async fn prepare_coordination(&self, id: &str) -> Result<()> {
        let saved = self.saved(id)?;
        self.authorized(saved.view.repository, &saved.profile, true)
            .await?;
        if let Some(expected) = &saved.coordination {
            let actual = if saved.profile == "native" {
                self.0
                    .resources
                    .driver
                    .materialize(
                        saved.view.repository,
                        expected.path.clone(),
                        self.0
                            .resources
                            .native_socket
                            .clone()
                            .ok_or(Error::Unavailable)?,
                    )
                    .await?
            } else {
                self.0
                    .resources
                    .driver
                    .inspect(expected.path.clone())
                    .await?
            };
            if actual.metadata != expected.metadata || actual.name != expected.name {
                return Err(Error::Stale);
            }
            return Ok(());
        }
        let path = saved.workspace.path.join("coordination");
        let workspace = if saved.profile == "native" {
            self.0
                .resources
                .driver
                .materialize(
                    saved.view.repository,
                    path,
                    self.0
                        .resources
                        .native_socket
                        .clone()
                        .ok_or(Error::Unavailable)?,
                )
                .await?
        } else {
            let (_, _, source) = self.conversation_home(Some(saved.view.repository)).await?;
            self.0
                .resources
                .driver
                .create(
                    source.clone(),
                    path,
                    source.operation,
                    source.commit.clone(),
                    format!("agent-coordination-{id}"),
                )
                .await?
        };
        self.update(|state| {
            state
                .sessions
                .get_mut(id)
                .ok_or(Error::Invalid)?
                .coordination = Some(workspace);
            Ok(())
        })
    }
}
