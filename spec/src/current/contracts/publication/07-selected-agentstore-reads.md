# Selected AgentStore reads

The shared immutable reader, catalog and projection engines now live in
jj-store-core under JST-ARCH-006. Native facades retain their existing audited
root selection and translate errors at the provider boundary. These engines
cannot publish a manifest, read an unselected native object or replace native
authorization; JST-PUB-001/004/005 and existing canonical record bytes retain
their interpretation.

The WAL layer exposes an audited `AgentStoreSnapshot` for one manifest-selected
root. Its exact transitive object count is available to higher-layer read-budget
enforcement together with the already verified immutable object map; this count
does not enumerate bucket state, stage an object, construct a delta or expose a
publication method. Root-pinned session paging keeps that snapshot alive across
later publications without changing the manifest, WAL, receipts, domain version
or repository position.

The publication facade re-exports the owned AgentStore identity and immutable
link types needed for service DTO conversion. Stable-session detail derives the
session catalog key, resolves it through `AgentStoreSnapshot::lookup` and reads
only the selected immutable session object in that audited closure. These facade
types and reads add no reverse service dependency or new durable encoding.

Bounded transcript reads accept only a manifest already selected by that
session, an exact source instance/generation, record limit, byte ceiling and
optional last sequence. `AgentStoreSnapshot::transcript_range` verifies linked
page and chunk kinds, stream identity and strictly increasing sequence before
returning sanitized immutable chunks. The higher-layer signed cursor pins this
existing audited snapshot; it does not add a manifest pointer, WAL record,
receipt or publication transition.

Bounded relation reads accept one typed session, tool, operation, revision or
checkpoint subject and a root-local page/offset cursor.
`AgentStoreSnapshot::relations` verifies the selected relation page chain and
returns only ordered checkpoint-version or association references. The
publication facade re-exports those relation, checkpoint and association model
types for service conversion without exposing a staging or publication method.
The higher-layer continuation binds the complete subject and existing audited
snapshot; it creates no durable cursor or new stored object.

Checkpoint collection uses the existing root-selected checkpoints catalog and
its radix continuation key. Current detail derives the stable checkpoint catalog
key and verifies that the selected object carries the same checkpoint identity.
Exact-version detail follows only typed checkpoint `supersedes` references from
that selected object, bounds unique traversal at 4,096 versions and rejects
cycles or identity substitution. These reads use the already audited AgentStore
closure and introduce no alternate object lookup, manifest pointer or
publication transition.

Session activity timing follows normalized event IDs into the exact observation
batch referenced by each selected event batch. It reports the minimum and
maximum matched `occurred_at_millis` only when they are available within the
4,096-event read bound; missing, mismatched or over-bound evidence reports no
timing. The derived fields are service presentation data and add no AgentStore
object, manifest, WAL or receipt encoding.

M32 moves the pure cross-domain target resolver beside the shared immutable
engines. JST-PUB-001/004/005/007/009 remain unchanged: native authorization,
manifest/WAL publication, original receipts, closure admission and unknown
outcomes stay under the native coordinator. Native missing-object errors retain
their neutral target-unavailable interpretation across the core boundary.
