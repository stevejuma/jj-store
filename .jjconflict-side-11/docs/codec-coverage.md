# Native contract coverage matrix

Audit baseline: jj-lib 0.45.1, revision
`7c41cdeb16b6b321c64e789a966b6adf723816a5`. This is a field/trait inventory;
implementation status lives only in the milestone ledger. M1 adds round-trip
coverage for every listed field rather than persisting native Serde output.

| Native value      | Complete field/variant coverage                                                                          | Encoding/identity obligation                                                            |
| ----------------- | -------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| Commit            | parents; predecessors; root_tree; conflict_labels; change_id; description; author; committer; secure_sig | Preserve parent/predecessor/merge order and all signature data; pinned ContentHash      |
| Signature         | name; email; timestamp                                                                                   | UTF-8 and exact timestamp                                                               |
| Timestamp         | timestamp (signed milliseconds); tz_offset (signed minutes)                                              | No truncation to seconds or UTC-only normalization                                      |
| SecureSig         | data; sig                                                                                                | Preserve both exact signed bytes and signature                                          |
| Tree              | sorted named entries                                                                                     | Validate repository components and duplicate/order errors                               |
| TreeValue         | File(id, executable, copy_id); Symlink; Tree; GitSubmodule                                               | Full oneof; no panic or default file fallback                                           |
| CopyHistory       | current_path; parents; salt                                                                              | Ordered copy parents and explicit placeholder handling                                  |
| CopyRecord        | target; target_commit; source; source_file; source_commit                                                | Reverse-topological streamed lookup; not a stored file blob                             |
| View              | head_ids; local_bookmarks; local_tags; remote_views; git_refs; git_heads; wc_commit_ids                  | Pinned content hashing; names and conflicted/absent refs preserved                      |
| RemoteView        | bookmarks; tags                                                                                          | Preserve remote-name nesting                                                            |
| RemoteRef         | target; state                                                                                            | New versus Tracked preserved                                                            |
| RefTarget         | ordered merge terms including None                                                                       | Never flatten absent/conflicted terms                                                   |
| Operation         | view_id; parents; metadata; commit_predecessors                                                          | None differs from Some(empty); retain invisible intermediate commits                    |
| OperationMetadata | time(start,end); description; hostname; username; is_snapshot; workspace_name; attributes                | Both time zones; absent workspace distinct from empty string                            |
| Roots             | root commit/change/view/operation; empty tree                                                            | 64-byte commit/op/view IDs; 16-byte change IDs; pinned zero roots and empty-tree vector |
| File              | exact bytes                                                                                              | Streamed BLAKE2b-512 identity; binary is valid                                          |
| Symlink           | UTF-8 target string                                                                                      | Pinned SimpleBackend hashing; no materialization traversal                              |

## Backend methods

name, commit_id_length, change_id_length, root_commit_id, root_change_id,
empty_tree_id, concurrency, read_file, write_file, read_symlink, write_symlink,
read_copy, write_copy, get_related_copies, read_tree, write_tree, read_commit,
write_commit, get_copy_records, gc. Copy relations include ancestors and their
descendants, deterministically children before parents. Submodule IDs preserve
native bytes but do not imply remote repository dependencies or checkout
support. GC is an advertised no-op until separately proven retention is
implemented.

## Operation stores

OpStore: name, root_operation_id, read_view, write_view, read_operation,
write_operation, resolve_operation_id_prefix, gc. Prefix indexing is bounded and
rebuildable; no per-query bucket LIST. Root handling remains explicit.

OpHeadsStore: name, update_op_heads, get_op_heads, lock. Head sets are nonempty;
remove-old/add-new semantics preserve siblings. A lock is an optimization, not
publication authority or fencing. Request receipts prevent late resurrection.

## Profile decision

Use SimpleBackend content hashes for ordinary commit/tree/file/symlink values,
SimpleOpStore content hashes for operations/views, and pinned root identities.
Use the pinned CopyHistory ContentHash for copy objects with a separate kind
namespace. Complete missing copy/submodule schemas in project-owned versioned
codecs. Physical envelopes carry an independent SHA-256 checksum. Do not sort
native ordered lists or replace hashes with protobuf-byte hashes.

The content profile is explicitly versioned with the JJ pin; future upstream
hash changes require a new representation/profile decision and compatibility
fixtures. Git import identity equivalence is not asserted.
