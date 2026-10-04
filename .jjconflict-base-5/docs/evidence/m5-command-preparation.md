# Typed native command preparation

This evidence covers internal command preparation in the enclosing Jujutsu
change. It does not qualify the public preview/apply/status workflow. Commands
stage complete native objects and an unpublished operation; the manifest remains
unchanged until a separate strict publication passes the existing authorization
and semantic guard.

The pinned jj-lib 0.45.1 transaction helpers implement New, Describe, and local
bookmark create/move/delete. New preserves explicit parent order and native
merge conflicts. Describe reparents descendants without changing their trees;
change identity, native author policy, bookmarks, logical workspaces and
predecessor records follow the native rewrite. New commits have an empty native
predecessor vector and are reported as created, never as rewrites. The stored
operation retains the original absent-versus-empty representation.

The executor loads compiled configuration defaults plus a fixed authenticated
principal identity and disables external signing. It never invokes an editor or
uses a physical checkout. It requires one exact head and rejects root/hidden
rewrite targets, unpublished dependencies, stale bookmark terms and unknown
logical workspaces. Historical committed objects remain valid explicit New
parents. A no-op Describe reports no commit/view effects.

Preparation has read, elapsed, metadata, work, response, immutable-object-count
and staged-byte limits. Its authority denies head publication. Its storage
wrapper accepts only absent-key writes under the exact repository native-object
prefix and permits reads only from the captured inventory or this command's own
attempted immutable keys. Native identity/checksum validation remains mandatory
for collision and unknown-outcome verification.

Nine native lifecycle tests exercise the real integration and authenticated
service over the in-memory test transport. Synchronous storage pauses verify
that canceled workers retain the shared permit until they exit, concurrent
publication makes preparation stale, and token revocation denies its result.
Separate tests prove a protected rewrite candidate cannot pass publication.
These fixtures are not RustFS conformance.

Run the focused lane with:

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked
```

Exact commands, failures and final aggregate outcomes are retained in E-M5-001
of the single [progress ledger](../../spec/src/milestones.md). Remaining M5
gates include durable canonical intent/receipts, public preview/apply/status,
ReconcileOperations, Abandon, ResolveConflicts, transport contracts and browser
mutations. M4 production RustFS/SSE/browser qualification remains separate.
