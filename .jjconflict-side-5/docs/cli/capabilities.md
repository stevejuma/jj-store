Report repository capabilities and their current verification scope.

Inside a Git-backed JJ repository, status, doctor and capabilities return the
version-2 local profile. Projects, tasks and comments run directly against the
companion metadata store without a native connection or server. Reading does not
initialize repository metadata or a local actor. Agent and review support is
reported separately as it is qualified. The native version-1 matrix remains
available in native repositories and outside a repository.

`jj store capabilities --format json` returns a versioned command matrix. Native
commit, operation and evolution histories are separate. Git transport commands
require a Git backend and are unsupported on native repositories. Destructive GC
remains disabled. A library test is not executable parity evidence: commands
without direct acceptance are reported as unqualified.

The executable matrix is qualified against jj-cli/jj-lib 0.45.1 on the current
Unix host. Semantic comparisons use an isolated upstream Git-backed reference;
native repositories contain no Git storage. Native IDs have their separate codec
conformance tests. Comparisons omit reference-only `@git` tracking refs.

| Commands or behavior                                      | Acceptance                                                                                        |
| --------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| edit, describe, new, log, diff, op log, evolog            | Executable lifecycle and restart fixtures                                                         |
| bookmark create/set/delete/list                           | Executable lifecycle, including conflicted bookmark reconciliation                                |
| rebase, squash, split, abandon, undo                      | Parent relationships, local bookmarks, descendant content and evolution compared with upstream    |
| multi-parent new; resolve --list; manual conflict editing | Three-sided conflicts and snapshot-based resolution compared with upstream                        |
| file show/list/chmod                                      | Binary bytes, symlink targets, executable modes and copied/renamed file snapshots                 |
| sign and unsign                                           | SSH Ed25519 signing, re-signing, verification and unsigned rewrites; requires a configured signer |
| concurrent operations                                     | Divergent descriptions and conflicted bookmarks reconcile without losing either accepted sibling  |
| Git transport and util gc                                 | Explicit rejection with unchanged publication snapshot                                            |

The named `native_cli_semantic_*` fixtures are in
`crates/jj-store-cli/tests/executable_native.rs`. They use memory-backed RPC and
are distinct from the retained RustFS cold-deployment reports. Filesystem copies
are content/snapshot evidence; complete native copy histories and submodules
have separate codec/backend fixtures because upstream exposes no file-copy
command in this release. External merge tools and other signing backends are not
covered by this matrix. Non-Unix platform qualification remains a release gate;
the ledger records completion.

The standalone TLS/RustFS recovery fixture uses a generated localhost
certificate with explicit client trust and no plaintext override. This qualifies
the production TLS transport on the current host; other deployment topologies
and release platforms remain separate gates.
