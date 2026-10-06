# Device-local integration registry

The CLI owns a versioned registry outside the checkout for each native
repository and canonical physical workspace. It persists one nonzero random
device identity and separate detection, desired enablement and setup state for
Codex, Cursor and OpenCode under an exclusive user-restricted lock. Atomic
synced replacement and strict repository/workspace validation preserve the
identity across restart and reject another authority opening the state.

Discovery searches the current PATH for each owned executable and executes a
two-second, 256-byte version probe. Only the exact qualified versions Codex
0.156.1, Cursor 3.22.7 and OpenCode 1.18.30 are supported. Missing executables,
other versions and failed probes are explicit states; stderr is discarded and
never enters the registry or command output. Explicit noninteractive enablement
may select only supported integrations, and dry-run does not modify durable
state. Enablement records desired policy while setup remains not installed.

Setup requires both enabled policy and exact-version support. It merges only the
owned command handlers into Codex or Cursor project hook documents and installs
one namespaced OpenCode project plugin. Before writing, the device registry
atomically stores exact original bytes and the original and owned SHA-256
hashes. Project files use bounded atomic replacement, preserve unrelated JSON
values and reject symlinked target directories. Restart recognizes either the
exact original-to-owned transition or the installed hash; any other bytes are
drift and block replacement. Dry-run writes nothing.

Owned removal is independent of current executable detection so an absent or
upgraded harness can still be disconnected safely. It accepts only the exact
owned target or an exact original already restored before interruption. The
former is atomically replaced by the recorded original, or deleted when setup
created it; the latter resumes ownership cleanup. A synchronized tombstone makes
record retirement restart-safe. Only after target and ownership cleanup does the
registry record disabled/not-installed state. Drift, symlinks and unexpected
ownership files fail closed, while dry-run and completed retries do not modify
project bytes.

The Codex contract follows `openai/codex` tag `rust-v0.156.1` at source revision
`b412ff32c417f855c2b2d1581b77058eed87c84b`; the OpenCode plugin contract follows
`anomalyco/opencode` tag `v1.18.30` at source revision
`3104c1428ec91f809e5ab86631300de41eb6952e`. Cursor uses the documented version-1
project hooks shape, but remains synthetic setup acceptance because the local
agent reports `2026.09.26-dd393fe` rather than qualified `3.22.7`. Drift-safe
removal and source-ordered exact publication are implemented. The M9.6 real-PATH
gate accepts the installed Codex and OpenCode versions, rejects the Cursor
drift, and proves side-effect-free setup previews in a disposable workspace. It
does not claim compatibility for the unqualified Cursor build.
