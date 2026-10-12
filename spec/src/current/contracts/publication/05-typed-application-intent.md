# Typed application intent

Typed application commands use a separate domain-separated intent profile. Their
identity binds repository, authenticated principal, request/publication IDs,
exact expected snapshot, semantic kind and bounded canonical command bytes.
Generated native operation/commit IDs, server timestamps and candidate result
bytes are excluded: several native preparations of the same original command may
differ in those values. Native and token-administration requests retain the
existing complete-request digest profile unchanged.

Only strict native head transitions may use application intent. The service must
construct canonical command bytes from validated owned command types, including
every user-selectable semantic input. Caller-supplied digests and raw
publication requests are not accepted by either transport. The coordinator still
authorizes the current principal, validates each uncommitted candidate's native
semantics and complete dependencies, and performs the same manifest CAS. No
intent reservation or candidate pointer establishes commitment.

Equivalent racing candidates return the one manifest-selected receipt. Receipt
lookup precedes stale checks and native semantic candidate validation. Different
canonical commands, expected snapshots or identity fields conflict under the
same retained request key. The accepted receipt contains the selected result;
retries do not synthesize results from the retry's candidate.

A bounded receipt reader can resolve a principal/request key through a checked
manifest snapshot without reconstructing native candidate IDs. It checks both
receipt indexes, exact identity and accepted epoch/position. Like selected
settings access, this internal storage interface requires current authorization
at the service boundary. Missing entries mean no selected receipt was observed,
not proof that an in-flight request was rejected. Checkpoints retain this
lookup.

The exact Snapshot value exports JSON Schema metadata for the typed HTTP
service. Schema derivation changes no serialization, hashing, accepted heads or
durable WAL record format; no HTTP transport dependency enters publication.

M26 optional library snapshots on Message, Conversation Submit and Set reaction
are part of the original typed intent. They stage immutable source/preview
closure with the authored use under the unchanged domain receipt and manifest
CAS. Source names are data-only provenance. Absent optional fields preserve
legacy encodings; directory edits or retirement cannot alter prepared retries.
