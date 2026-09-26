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
