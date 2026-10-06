# Captured application content search

Native application content search retains bounded data-only indexes keyed by
repository, epoch and immutable Thread/Work roots. Validate complete selected
domains once per retained generation, admit normalized body growth before
allocation, and retain separately ordered recent and text-search keys. First
queries filter these compact fields and merge existing orders; continuations
retain matching keys. Hydrate excerpts only for returned hits, with bounded
Unicode-safe snippet scanning. Unchanged roots survive new filters and fresh
manifest envelopes; changed roots rebuild independently. Current tombstones,
archived Thread rules, task states, scoped matching and stable ordering remain
unchanged. Warm and continued pages recheck viewer, inventory admission, epoch
and current Read permission. Root indexes and query keys have separate bounded
retention charges; neither search cache nor cursor becomes repository authority.

Profile-aware connection and route restoration remain shared by repository
navigation, Threads and entity search. Global native search skips local
Git-backed profiles without issuing native API requests and reports their
unsupported search explicitly. The local profile's existing capability gates
remain in effect; a source change never fabricates native Thread authority.

`GET /v1/repos/{repo}/content/search` searches the complete bounded current
Thread and WorkStore roots, independently of the selected code operation and
Tantivy file scope. It includes current visible human messages and replies,
project/task descriptions and current comments. Edits replace prior content and
tombstones remove it. Empty queries return recent active entity summaries.
Continuations retain the immutable result projection, bind every filter and page
size, expire after five minutes and reauthorize the same viewer and repository
before every page. Epoch replacement requires an explicit refresh. No body,
count or identity is returned before repository admission. Full source IDs and
revisions accompany results; navigation preserves their native meaning.

The browser fans out at concurrency two over all connected authorized sources,
clears protected results on session replacement/disconnect, and shows per-source
failures. Section, repository, Thread, project and task scopes filter on the
server. Code results keep their own operation-selected Tantivy snapshots.
