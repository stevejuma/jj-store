# Version 1 persistence codec

Metadata uses explicit project-owned, deny-unknown-field JSON schemas. Canonical
compact serialization fixes field order; maps and sets serialize in native key
order, while parents and merge terms retain their original order. Decoding
rejects noncanonical bytes, duplicates, invalid ID widths, invalid paths,
invalid merge arity, and unknown variants. Native structs are never serialized
through their generic Serde implementation. File payloads remain raw binary
bytes.

The fixed envelope carries magic, envelope version, native profile, object kind,
64-byte native ID, payload length, and SHA-256 of the preceding header and
payload. Decoders check caller limits before allocating and check the expected
kind and ID as well as semantic identity after decoding. Native roots are
virtual zero IDs; ordinary stored objects use content hashes even when their
contents equal a root value.

Publication records use their own SHA-256 RecordDigest and IntentDigest types;
epochs, principals and attempt IDs are distinct 128-bit types. Their
serialization does not alter the frozen native envelope/profile. Typed object
keys are encoded with an explicit kind tag in control records.

Writable replicas add three further owned identities without changing native
object identity. `ReplicaId` is the durable identity of one installation,
`WriterEpoch` identifies one deliberate origin-stream lifetime, and
`OriginDigest` is the SHA-256 digest of one canonical original publication
envelope. An origin key combines repository, replica, writer epoch and accepted
sequence. These values remain distinct from `EpochId`, `WalPosition`, native
operation and object IDs, record digests, request IDs and publication IDs. A
logical origin is forwarded unchanged, while each accepting replica records a
new local delivery position under its own replica and log epoch.

Token IDs are distinct 128-bit identifiers, independent of principal IDs and
publication/request identities. Bearer secrets and verifiers are service-owned
credential types and never native object identities.

The local Git profile adds a distinct 20-byte `GitSha1Id` and 32-byte
`CodeCapsuleId`. Profile-tagged commit and object references never pad or cast
Git identities into native IDs. Source operation/view IDs keep the pinned
64-byte upstream identity, including exact synthetic root sentinels. Additive
portable review anchors use schema version 2 and retain their source repository
and evidence identity independently of a destination home. Native V1 codecs,
envelopes and anchors retain their existing frozen interpretation.

Portable agent association and checkpoint schema V2 carries source-profile
evidence separately from shared non-code facts. Ordered evidence sets use the
owned typed identities without changing their serialization or native V1
association/checkpoint layouts. A Git commit paired with native evidence, or
another mismatched profile, is invalid.

The native V1 payload types are also exported as complete owned read DTOs. Their
field ordering, serialization and identity conversions remain frozen and covered
by the existing codec fixtures. API wrappers version their use of this native
profile; exposing fields does not permit generic upstream serialization.

ConflictFingerprint is a distinct 64-byte BLAKE2b digest of the versioned,
domain-separated conflict descriptor input. It is neither a native object ID nor
an envelope checksum. Native identity and persistence formats are unchanged.

API schema traits describe the existing canonical hex serialization for owned
IDs, object keys, copy references and publication positions. They do not alter
native IDs, codecs or persisted payloads. Schema validation does not replace
codec integrity and semantic checks.

ConflictUploadId is a canonical 32-byte lowercase-hex staging-record digest,
separate from native file IDs and envelope checksums. Its V1 SHA-256 domain is
`jj-store/conflict-upload/v1\0`; canonical field order, scope binding and an
independent golden vector are frozen in service upload tests. It is neither a
native object ID nor a repository access capability.
