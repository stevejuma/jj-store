# Installed hook ingestion

`jj agent hook <codex|cursor|opencode>` is the owned, noninteractive ingestion
entrypoint installed by setup. It accepts no caller-selected repository or
source identity. Before sanitization it reads the device registry and verifies
the repository, canonical workspace, explicit enablement, exact qualified
version, recorded installed state and current owned project-file hash. A missing
compiled-rule cache fails closed and does not trigger a cold hook-time build.

Standard input is bounded to four MiB plus one overflow byte. After in-memory
sanitization, a dedicated bounded lock records the local observation time and
allocates a strictly increasing native sequence under one stable random source
instance and generation. The source reservation is atomically synchronized
before spool enqueue; a failed or interrupted enqueue can leave an explicit
sequence gap, while restart and unknown outcomes cannot reuse the sequence.
Batch and observation IDs are domain-separated hashes of source identity,
generation and sequence.

Codex and Cursor observations use local-hook trust; OpenCode observations use
local-plugin trust. Every batch records the exact Kingfisher policy and matching
version-bound normalizer profile. Concurrent hook processes share the allocator
without opening the registry's exclusive administrative lock. Raw input,
findings and secret-bearing errors are never written or logged. Pending batches
remain in the existing capture spool until the hook's bounded best-effort
publication worker prepares and commits their exact AgentStore delta and receipt
identity. A server outage or three-second hook budget expiry preserves the
capture and defers another attempt to a later hook invocation.
