# Exact review anchors

`jj-review` owns fixed-width stable identifiers for review threads, comments,
actions and work requests, plus separate SHA-256 identifiers for anchors,
comment revisions, thread events, snapshots, work-request versions and roots.
Stable workflow identities never substitute for immutable version identities.
All identifiers use canonical lowercase hexadecimal encoding.

`ArtifactAnchorV1` binds one repository and exactly one code text, whole code
artifact, agent text, whole agent artifact, whole agent session, revision,
operation or repository target. Code anchors carry full operation and commit
IDs, optional change IDs, repository-relative paths and the exact native
file/object identity. Diff text anchors additionally carry an explicit
before/after operation, commit and path pair; the selected side must equal the
anchor's primary identity. Whole native artifacts accept matching file or
symlink object kinds. External-width gitlink identities have no owned native
object key and therefore fail closed in v1.

Text selections use canonical decimal-string UTF-8 byte offsets and a SHA-256
digest of the selected bytes. Empty, reversed, out-of-bounds, non-UTF-8-boundary
or digest-mismatched selections are invalid. No line number, display text or
search result can replace this byte-level check.

The additive `agent_session` target binds the stable session identity and one
exact immutable session-snapshot reference. It retains that snapshot and its
complete AgentStore closure through existing review dependency traversal. Native
staging and cold reads verify that the referenced snapshot belongs to that
session; wrong kinds, missing snapshots and mismatched identities fail closed.
Existing V1 anchor encodings and hashes remain unchanged. Filesystem-local agent
targets remain unsupported. Session comments are human ReviewStore comments,
never agent prompts, capture edits, tool replies or controller dispatch.

Other agent anchors pin an immutable session snapshot plus the exact tool
attempt, message or plan owner snapshot. Text targets also pin the block
version, content object, content part, source coverage and representation,
sanitizer and normalizer versions. Kind mismatches fail closed. Anchor
dependency traversal enumerates every native or AgentStore object needed for
original-context reads; stable entity IDs and optional change IDs remain lookup
facts rather than strong object edges.

The additive portable V2 anchor represents retained AgentStore sessions and tool
invocations directly. It never invents a tool attempt or message/block identity
when capture did not retain one. A tool target binds the session ID and
snapshot, tool-call ID and exact tool snapshot. A text target additionally binds
the stored input/output content reference and a validated UTF-8 selection over
the original sanitized payload bytes, without reformatting them. A session
target binds its exact session snapshot. Existing V1 anchor bytes and decoding
remain unchanged.

Portable agent structural validation requires the tool to occur under that exact
ID and snapshot in the named session and requires both stable IDs to match the
tool record. Text validation requires the content reference to equal the
selected tool input/output and verifies the selected byte digest. The read port
must supply canonical repository-bound records from a qualified retained
selection; structural validation alone cannot establish publication, access or
retention. Portable code evidence must bind the anchor repository, including
both sides of a diff. Providers continue rejecting portable publication until
their immutable retention, shared command/audit and receipt gates pass.

The local provider backs portable agent structural inspection with its
explicitly companion-pinned canonical reader. Only records in that selected
AgentStore closure are admitted; a staged Git blob or future session version
does not become visible through an earlier pin. Inspection preserves operation
heads and selected metadata across restart and ordinary Git GC, and remains
separate from review publication and complete historical retention.

Canonical JSON encoding rejects unknown fields, alternate byte-offset forms and
noncanonical round trips. The anchor ID hashes those validated canonical bytes.
The v1 resolver maps an immutable anchor only against an explicitly identified
target supplied through a narrow read port. Callers provide retained original
bytes, bounded target bytes and typed validated predecessor, rename, copy or
agent-block lineage. A same `ChangeId` never substitutes for an exact revision
or for lineage evidence.

The additive `portable_code_text` and `portable_code_artifact` command targets
embed the existing portable V2 code selectors in the shared review envelope.
Existing target tags, native bytes and hashes are unchanged. Exact capsule,
profile, repository, commit, path and blob identity are mandatory. The local
provider checks the physical retained commit/tree/path before admitting bytes;
text selection hashes validate against those original bytes. Both diff capsule
references are retained. Native providers continue refusing portable evidence
until their carrier is separately qualified; existing native code anchors remain
native. Conversion also continues refusing unsupported source carriers.
