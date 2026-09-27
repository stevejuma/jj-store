# Identity and native codecs

The upstream SimpleBackend cannot be adopted wholesale: copies are unsupported
and submodule serialization panics. Complete these variants in the project
codec. Full native values, not generic Serde JSON, determine persistence.
Preserve invisible intermediate commits needed by evolution. The profile is
verified in M0 before user data is stored; new-repository compatibility is not a
promise of Git-backend import compatibility.

## Requirements

| ID         | Requirement                                                                                                                                                                                     |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ID-001 | Native IDs, repository IDs, request IDs, publication IDs, WAL positions, and envelope checksums MUST remain distinct typed concepts.                                                            |
| JST-ID-002 | The v1 identity profile MUST use pinned SimpleBackend content identity and SimpleOpStore operation/view hashing and roots, with golden fixtures and separate envelope integrity digests.        |
| JST-ID-003 | Explicit versioned codecs MUST preserve every pinned field and variant, including copy histories, submodules, signatures, merge ordering, git heads, and None versus empty commit_predecessors. |
| JST-ID-004 | Unknown formats, corrupt bytes, identity collisions, or missing historical dependencies MUST fail closed rather than produce an incomplete successful graph.                                    |
| JST-ID-005 | Codec upgrades MUST retain readable immutable historical representations; migration MUST NOT silently remap native IDs.                                                                         |

## Pinned coverage

The [codec inventory](../../docs/codec-coverage.md) enumerates all pinned fields
and required methods. Copy IDs use the pinned CopyHistory content hash in their
own kind namespace. Submodule links are native IDs, not transitive dependencies
on another repository. Generic native JSON omits several identity-bearing
fields.

## M0 extension qualification

Pinned hash fixtures distinguish absent, empty, and intermediate predecessor
maps, preserve timestamp offsets, and verify special roots and the empty-tree
identity against upstream SimpleBackend/SimpleOpStore. Full codecs remain M1.

## Version 1 persistence codec

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

Token IDs are distinct 128-bit identifiers, independent of principal IDs and
publication/request identities. Bearer secrets and verifiers are service-owned
credential types and never native object identities.

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
