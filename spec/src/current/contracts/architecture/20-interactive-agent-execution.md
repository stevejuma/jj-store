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
