# Vault history search projection

JST-ARCH-007 keeps the reusable vault boundary server-independent. Native
retained-object and delta reads stay in jj-store-jj. jj-vault owns the
disposable Tantivy adapter, native content references, resumable frontier and
query DTOs. Core storage, model and WAL gain no Tantivy dependency or index
authority.

The host chooses private projection placement and the admitted vault identity.
Its bounded worker performs index/query work independently of capture, editor
writer guards, networking and renderer execution. Only successful checkpoint
publication schedules new native versions; indexing never snapshots files.

JST-LOCAL-007 governs changed-version text/path/metadata results and partial
coverage. Index batches commit their durable resume markers together. Checkpoint
parents preserve graph ancestry; immutable content IDs deduplicate file bodies.
A missing or corrupt projection rebuilds from retained native objects. Query
rows do not authorize source access, restoration or remote publication.

Ordinary native queries admit an existing committed projection snapshot. They
wait at most five seconds for an active publisher, retain explicit Busy on
sustained contention, and never initialize or repair projection bytes. The
selected manual reader pins its generation and releases publisher admission
before source-membership checks. Capture and background scheduling remain
independent; cached progress completion is not a query-admission barrier.
