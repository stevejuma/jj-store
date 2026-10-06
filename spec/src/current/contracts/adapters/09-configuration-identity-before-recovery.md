# Configuration identity before recovery

The runner validates a projection's accepted identity before acquiring writer
ownership or reading its remote cursor. Native Tantivy extraction binds that
identity to its complete canonical configuration. A caller cannot label one
extractor with another configuration digest, implementation or schema.

Recovery checks that binding before exposing a generation. An incompatible local
artifact is a rebuild candidate and remains unavailable to new readers,
including when the replacement build fails. Rebuilding at the same native
position produces an artifact for the new configuration. Existing compatible
pins retain their immutable generation; native repository data and IDs do not
change when source projection exclusions change.

Server composition creates one runner per explicitly configured repository and
uses the canonical native extraction identity. Active index paths are derived
from full repository IDs beneath private server state; users cannot supply paths
from repository content. Configuring no source scopes indexes metadata only.
Authenticated runners hold their repository token only in memory and reauthorize
on every batch. LocalTrusted runners use a stable installation-derived read-only
principal and share the same repository, snapshot and resource checks.
Shared-mode configuration reload requires an explicit server restart.
LocalTrusted Admin Search settings additionally support the isolated Apply
operation described below. Local cache loss rebuilds from selected history
regardless of an advanced remote adapter cursor.

A graceful runner shutdown waits until retained callback contexts release native
and blocking work before releasing writer ownership. A forced task drop still
cancels workers; their context clones retain ownership until actual exit.
Rebuild equivalence compares document contents and native source positions;
equal-score ordering may differ in a newly built index. Pages within one pinned
generation retain their exact ordering.

A closed runner health channel marks search degraded even when its last index
remains readable. A terminated worker cannot leave a permanently healthy status.
