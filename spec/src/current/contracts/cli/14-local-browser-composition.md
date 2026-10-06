# Local browser composition

Explicit local-folder registration is composed through a read-only JJ verifier.
It supports canonical colocated and linked Git-backed JJ workspaces, never reads
executable repository settings and never snapshots checkout files. Server-owned
private request journals retain initialization receipts before discovery. The
filesystem-only serving mode and local browser endpoints are separately
qualified under M13.4.

The CLI's isolated ordinary-JJ fixtures qualify operation-pinned server reads
against the explicit development binary. They retain dirty checkout files and
verify graph, binary, conflict, root and multi-head behavior without invoking
repository-configured programs or installing the CLI. Provider tests do not
qualify filesystem server startup or browser integration.

`jj-store init --local --storage filesystem --state-dir <new directory>` creates
private installation/catalog/operator configuration without RustFS credentials,
volume or RPC socket. The default remains native embedded storage. Filesystem
mode requires literal-loopback LocalTrusted administration and rejects native
backup/search/replication configuration. Native routes report unsupported
authority rather than constructing a placeholder object store.

Local browser WorkStore writes use the same shared-JJ device actor as
`jj project`, `jj task` and `jj task comment`. The CLI can look up browser
requests and returns their original committed receipts, independently of the
code operation or later metadata publications.

Local browser AgentStore reads present only records already selected through
local CLI capture/provenance publication. Opening the app does not enable
capture, install hooks or create observations; independently pinned browser
selections can remain older than subsequent CLI captures until explicit refresh.

Local browser admission retains a durably proposed repository identity before
companion initialization. Existing CLI actor and initialization receipts are
reused after restart; explicit reconnect revalidates a moved folder without
changing portable metadata or installing hooks.

Filesystem browser startup admits concurrent snapshot, code and WorkStore reads
through a bounded worker queue with the configured request deadline. Ordinary
parallel page loading does not require a native service. Cold isolated CLI/HTTP
fixtures qualify this startup boundary separately from provider-only tests.

The local browser physical-file Save is independent from CLI code commands. CLI
`--ignore-working-copy` reads must observe unchanged operations, bookmarks and
companion selections after Save; later explicit foreground Snapshot remains the
only way those physical edits enter recorded code state.
