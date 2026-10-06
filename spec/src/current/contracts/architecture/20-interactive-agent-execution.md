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
