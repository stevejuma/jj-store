# Interactive agent execution

The LocalTrusted service owns an optional process supervisor using a configured
jj-acp binary over inherited stdio. Service interfaces expose execution and
workspace capabilities without importing ACP implementation. Storage/model/WAL
stay independent. Saved setups and device lifecycle journals live outside
checkouts; sanitized repository history remains selected by AgentStore
authority.

Publication selects at most 128 consecutive observations and four MiB of
payload. Its end sequence is journaled before spool admission; batch and
observation IDs derive from the original session and exact range. Recovery first
drains the retained spool receipt, then advances the committed cursor. A
completion before cursor persistence reuses the same selected range, including
after more live events arrive. No retry changes the publication intent or
replays a prompt.

Privacy rules compile on a detached preparation thread that owns no subprocess
or publication. Service shutdown does not wait for unused scanner preparation;
the stopping supervisor discards a late result.

The application owns a fixed runtime catalog outside jj-acp. Explicit Codex
installation runs resolved npm without a shell into a fresh private request
directory, with exact adapter/CLI versions, fixed public registry, no lifecycle
scripts, private cache/config and bounded time/disk use. Verified executable and
version probes precede atomic journal selection; failed updates keep the
previous selection. Discovery and launch share the selected private bin path
plus resolved Node and conventional device tool directories. No npx-cache search
is used. Node provisioning and a managed Cursor installer remain separate
qualification.

M24 adds
[reusable agent conversations](../collaboration/37-reusable-agent-conversations.md)
under JST-COLLAB-034 through JST-COLLAB-036. Existing M23 qualification remains
unchanged.

The server owns Markdown context assembly. The shared collaboration model owns
the bounded Markdown-aware answer protocol parser so native and local
publication perform the same validation. One sanitized document supplies ACP
wire context and the retained UI inspector; service reply selection remains
independent of presentation and controller runtimes. Coordination checkouts are
separate from code attachments and code change summaries. The ACP protocol and
storage/model/WAL boundaries are unchanged.

Frozen comment publication precedes prompt preparation and dispatch. The
supervisor journals publication requests, selected batches and pending exact
answer links. Only a definitive stale refusal followed by a NotKnown receipt
read permits a new answer-link attempt with an updated thread precondition; the
logical link and answer range stay unchanged. Unknown outcomes never change the
request. This journal does not replace native or companion authority.
