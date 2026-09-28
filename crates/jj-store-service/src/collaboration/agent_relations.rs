use super::*;
use jj_store_wal::publication::{
    AgentObjectKind, AgentObjectRef, AgentRelationCursor, AgentRelationRange,
};

impl AgentSessionPager {
    pub async fn begin_relations(
        &self,
        session: Arc<NativeSession>,
        request: AgentRelationRequest,
    ) -> Result<AgentRelationPage> {
        request.validate()?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.begin_relations_inner(session, request),
        )
        .await
    }

    async fn begin_relations_inner(
        &self,
        session: Arc<NativeSession>,
        request: AgentRelationRequest,
    ) -> Result<AgentRelationPage> {
        let pin = self.pin(&session).await?;
        let range = pin
            .store
            .relations(&request.subject.wal(), None, request.limit)
            .await
            .map_err(map_wal)?;
        authorize_epoch(&session, pin.snapshot.epoch).await?;
        let snapshot = pin.snapshot.clone();
        let next_cursor = range
            .next
            .map(|next| {
                self.cursors
                    .retain(pin, request.limit, request.position(next))
            })
            .transpose()?;
        relation_page(snapshot, request.subject, range, next_cursor)
    }

    pub async fn resume_relations(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        request: AgentRelationRequest,
    ) -> Result<AgentRelationPage> {
        request.validate()?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.resume_relations_inner(session, cursor, request),
        )
        .await
    }

    async fn resume_relations_inner(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        request: AgentRelationRequest,
    ) -> Result<AgentRelationPage> {
        authorize(&session).await?;
        let (claims, entry) = self
            .cursors
            .lookup(session.repository, cursor, request.limit)?;
        let Position::Relations {
            subject,
            page,
            offset,
        } = claims.position
        else {
            return Err(Error::InvalidCursor);
        };
        if subject != request.subject {
            return Err(Error::InvalidCursor);
        }
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let range = entry
            .pin
            .store
            .relations(
                &subject.wal(),
                Some(AgentRelationCursor { page, offset }),
                request.limit,
            )
            .await
            .map_err(map_wal)?;
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let next_cursor = self.cursors.next(
            &claims,
            &entry,
            range.next.map(|next| request.position(next)),
        )?;
        relation_page(entry.pin.snapshot, subject, range, next_cursor)
    }
}

#[derive(Clone, Copy)]
pub struct AgentRelationRequest {
    pub subject: AgentRelationSubject,
    pub limit: usize,
}

impl AgentRelationRequest {
    fn validate(self) -> Result<()> {
        valid_limit(self.limit)
    }

    fn position(self, cursor: AgentRelationCursor) -> Position {
        Position::Relations {
            subject: self.subject,
            page: cursor.page,
            offset: cursor.offset,
        }
    }
}

fn relation_page(
    snapshot: AgentReadSnapshot,
    subject: AgentRelationSubject,
    range: AgentRelationRange,
    next_cursor: Option<String>,
) -> Result<AgentRelationPage> {
    let relations = range
        .selections
        .into_iter()
        .map(relation_link)
        .collect::<Result<_>>()?;
    Ok(AgentRelationPage {
        version: 1,
        snapshot,
        subject,
        relations,
        next_cursor,
    })
}

fn relation_link(reference: AgentObjectRef) -> Result<AgentRelationLink> {
    let kind = match reference.kind {
        AgentObjectKind::CheckpointVersion => AgentRelationKind::Checkpoint,
        AgentObjectKind::Association => AgentRelationKind::Association,
        _ => return Err(Error::Corrupt),
    };
    Ok(AgentRelationLink {
        kind,
        object_id: reference.id,
    })
}
