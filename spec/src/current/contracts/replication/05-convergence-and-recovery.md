# Convergence and recovery

The hub is another writable replica and uses the same receiver. A pure import
does not mint an origin, so forwarding through several peers converges by the
accepted-origin catalog instead of producing echo publications. Equal accepted
origin sets and compatible reducers produce equivalent domain state while local
manifests, delivery order, search generations and unresolved conflicts may
differ.

Initial hydration and existing attachment migration are explicitly online.
Format-2 repositories without origins receive one source-authored seed
checkpoint that inventories their accepted code, agent and review state without
inventing historical causality. A replica restored from old storage uses a new
stream identity unless continuity is proved and fenced.

The seed is the first origin of a newly allocated source stream and is selected
by the same manifest CAS as its WAL transaction, receipt, accepted-origin
catalog and causal prefix. It names every selected native operation head and the
complete current AgentStore and ReviewStore roots. Receivers recognize only this
exact four-domain first-origin shape as a baseline, verify its complete closure,
and merge every selected catalog fact with any local state. Empty domain
catalogs remain an explicit empty baseline. Retrying the same seed request
returns its durable receipt; a repository that already has a different
replication root cannot be reseeded in place.

Normal local commands never wait for the worker. A receiver acknowledges only
after its own manifest CAS is durable. Lost responses are resolved by querying
or replaying the same origin keys. A feed horizon gap requires a compatible
checkpoint plus tail; a replica with pending local work merges that knowledge
through normal import rather than replacing its roots.

Negotiation reports the oldest readable feed cursor and, when older positions
have been compacted, a compatible complete seed checkpoint. The checkpoint is
bound to the source replica, log epoch, source manifest digest, seed origin and
captured WAL position. Bootstrap transfers and applies the seed through the
ordinary receiver, advances peer progress only after the destination receipt is
durable, and then scans the tail to the finite upper bound captured by the same
negotiation. A mismatched descriptor, missing three-root closure, cursor newer
than the source upper bound or unavailable compatible checkpoint fails closed.
The destination manifest, replica stream and workspace metadata are never copied
from the source.

An existing physical workspace migrates through a durable, secret-free client
journal outside the checkout. The journal binds the original cache and remote
connection, the destination LocalTrusted connection and cache, the adopted
workspace identity, and stable local-repository creation identity. Migration
creates or verifies the independent local repository, pulls a complete seed and
tail through the ordinary receiver, initializes a new client cache, and verifies
that the existing working-copy operation loads from local storage. Only then may
it preserve the exact old `.jj/repo` descriptor as a recoverable backup and
atomically replace that descriptor with the local cache path. It MUST NOT run a
checkout, snapshot, working-copy update, reconciliation, hook, or external
adapter during this handoff. Retries reuse the journal and accepted origins;
unsnapshotted files and `.jj/working_copy` bytes remain untouched.
