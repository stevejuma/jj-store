# Local AgentStore browser selection

Recorded local agent reads require an explicit committed companion metadata
selection. They reuse audited AgentStore closure readers and shared sanitized
activity/timing/transcript presentation. Session and transcript identities
remain stable and immutable respectively. The shared timing projection may
recover qualified Codex JSONL RFC 3339 timestamps from exact matched sanitized
evidence when the observation timestamp field is absent. It preserves the
selected companion pin, including after later capture. Tool durations use only
ordered exact selected call/result or lifecycle endpoints, remain absent for
incomplete or ambiguous evidence and retain recorded outcome state. This
read-only projection neither rewrites recorded evidence nor requires reimport.
Portable code associations retain their exact source repository, Git SHA-1
commits, operations and evidence capsules; source identity is never substituted
with the metadata home. Browser relation queries are bounded projections of
selected associations and report continuation when more qualified records exist.
Reads cannot initialize metadata or record agents.

Companion-pinned AgentStore message activity and transcript pages expose
optional occurrence milliseconds from their exact selected evidence using the
shared qualified timestamp projection. Historical pins retain original message
times after later capture; reads do not publish or infer times from ingestion.

Local freshness streams revalidate device bindings and perform bounded checks of
operation heads and the selected companion ref. They announce current selections
without reconciling heads or changing working files. The browser compares
reconnect announcements against its retained selections and automatically
reloads new committed data, including changes missed while disconnected.

The browser selects transports by discovered profile and availability. Folder
controls persist the exact pending Add request in device browser storage before
dispatch. Reload checks its outcome; retry reuses the same intent. Removal and
reconnect retain repository and project identities. Browser collaboration roots
and current-head views refresh automatically; explicit historical code
selections remain pinned.

Local browser read transports reuse the existing bounded busy-retry policy for
shared read-worker admission. A retry retains the exact operation, companion
selection and query, including its continuation; it never captures a newer
snapshot. Only read operations retry automatically. Registration and domain
commands retain their explicit receipt and unknown-outcome handling.
