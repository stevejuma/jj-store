# Pinned local WorkStore transport

Local repository-card ReviewStore activity uses an explicit committed companion
pin and current folder admission, then reads only selected thread/event facts
for a bounded first catalog window. Full descriptor/retained-object closure
admission remains mandatory before deriving the count and maximum timestamp.
Activity responses do not hydrate comment or anchor presentation and mark the
count incomplete whenever the selected window has more catalog entries.

Local browser WorkStore pages require an explicit committed companion metadata
selection, independently from the code operation. Initial pages and resumed
queries validate that selection against bounded current committed history and
reject staged or unrelated commits. Filtered summaries use the same complete
root projection, ordering, counts and bounds as native WorkStore. Browser
continuations bind repository, binding generation, metadata selection and exact
query; changed filters or another repository fail closed.

Local task activity pages use the same complete-root body-free
repository/project groups as native WorkStore. Their explicit companion commit
is admitted through current committed history, with the existing folder binding
checks and signed query-specific continuations. Page work copies only selected
aggregate rows. Work browse/activity/detail facades share a process-local,
data-only root projection after full companion admission and a fresh admitted
Work-blob read. The key includes repository, Work object and SHA-256 of its
bytes; metadata commits, versions and binding checks remain request-bound.
Missing objects, tighter source limits, unrelated/staged selections or replaced
identities MUST fail before cached data is returned. No identity-only
observation can substitute for a domain Snapshot.

Local Work projection retention is bounded to 32 roots, 256 MiB and five minutes
of idle time, with eight exact-query key sets per root. Construction is
serialized outside the root-cache map lock and reserves conservative
decoding/projection space before allocation. A 512-MiB process derivation
allowance accounts both construction and resident lifetimes; eviction releases a
resident charge only after its last reader exits. Each query-key set has its own
live charge, reserved before filtering/allocation and retained until its last
reader exits. Root retention includes the allowance for eight query slots;
evicted key sets still held by readers remain under the process live allowance.
Oversized retention may return the validated result request-locally under the
same live allowance; pressure may return Busy while active readers occupy it.
Ordinary WorkStore capacities, companion admission, formats and publication
semantics remain unchanged. Domain/retained-closure admission itself is still
required on every request and is separate from this projection reuse.

Local browser edits and status use the existing CLI device actor and original
typed WorkStore request. Receipt replay precedes stale entity checks. Transport
timeouts after dispatch retain the exact request as an unknown outcome; removal
or replacement of a folder binding cannot grant access to a retained page.

Admitted browser collaboration facades bind the original repository identity.
Every write checks that identity before device actor initialization and on each
publication retry; a replaced companion root is stale rather than another home
to which the original command may be applied.
