# Durable pending capture

`CaptureCursorV1` records the device-local integration and source identity,
nonzero generation, bounded parser version, optional native session key and the
source-specific accepted position. It is distinct from the published source
state. A native-sequence cursor can acknowledge a batch only at that batch's
last accepted observation.

The local capture spool uses short exclusive mutation locks outside repository
checkouts. Its directories are user-restricted on Unix, its entry count and
aggregate bytes are bounded, and every entry is a versioned regular file named
by the capture batch ID. Enqueue validates the supported normalizer profile,
sanitized batch, cursor and repository binding, writes through an atomically
renamed temporary file, synchronizes both file and directory and only then
returns the cursor acknowledgement. Exact retries are idempotent; a changed
repository, profile, cursor or batch under the same ID fails closed.

Preparing publication atomically replaces that entry with the exact validated
`AgentDeltaRequest`, including stable principal, request and publication IDs.
Restart discards only unacknowledged atomic temporary files, then reopens and
validates every pending entry. Corruption, symlinks, unexpected files,
concurrent mutation and byte-limit exhaustion fail closed. Completion removes
only a prepared entry whose request and publication IDs match the committed
outcome, then synchronizes the directory. An unprepared capture cannot be
discarded through the completion path.

Native capture publication is two phase. A bounded canonical prepare request
sends the supported profile, sanitized batch, stable request/publication IDs and
result to the repository service. The server derives the principal and
activation time from the admitted peer, pins the current AgentStore root and
returns the exact validated delta request; the device spool must persist those
bytes before publication. The second call submits only that prepared request.
Committed receipts and unknown outcomes retain the same identities, while a
different peer principal, changed retry or stale same-source edit is rejected.
Preparing again after an unpersisted response may stage a fresh intent, but a
persisted prepared entry is never restaged under its existing receipt identity.
The CLI worker validates every pending entry against the active repository
before its first network call. It orders by integration, source instance,
generation, first source sequence and batch ID, derives domain-separated stable
request and publication IDs from repository plus batch ID, and uses a bounded
batch acknowledgement as the receipt result. Only a matching committed receipt
removes the synchronized prepared entry. Unknown outcomes stop the drain;
transport errors, staleness, rejection and mismatched replies retain the exact
entry and prevent later captures from passing it.

Native and local recovery share a separate exclusive drain lease held from queue
selection through the final outcome. The drain lease is released on
cancellation, error or process exit. Queue reads, enqueue, preparation and
completion each hold the mutation lock only around bounded local I/O, never
across publication or other awaits. New hook processes can durably enqueue while
either native prepare or publication is waiting; they cannot start a competing
drain. Each drain uses its initial bounded queue snapshot and leaves later
arrivals for the next pass. Recovery of orphan temporary files also holds the
mutation lock, so opening another handle cannot remove a writer's live temporary
file. Both locks are device-local coordination only; exact prepared requests and
matching receipts remain the authority for retry and completion.

New handles retain a shared legacy `capture.lock` guard for their lifetime.
Older binaries require that guard exclusively and therefore cannot bypass the
new `mutation.lock` and `drain.lock` protocol. If an older writer already owns
the spool, a new handle fails closed before queue inspection or mutation. The
pending V1 record format is unchanged.
