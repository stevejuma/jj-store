# Supervised desktop launch

The additive desktop run flags use the existing foreground composition and
format-2 installation. They require LocalTrusted embedded storage, literal
loopback and a pipe on stdin. Launch-only bundle-path and origin overrides never
persist or alter credentials. Owner EOF enters the same bounded shutdown as
interrupt/termination. See [desktop](../desktop/index.md).

The watchdog retains the duplicated owner descriptor in Tokio's safe owned Unix
pipe receiver, which verifies read access and registers nonblocking I/O. It
drains any owner bytes and enters the same bounded shutdown on EOF or a read
error. This preserves JST-CLI-002/JST-CLI-004 and the crate's unsafe-code ban;
it does not use the deprecated generic AsyncFd constructor or suppress strict
compiler diagnostics.

Native browser qualification MUST verify that authenticated query metadata omits
host identity hints. Registered-folder identity fallback remains an adapter
read; it MUST NOT invoke installed JJ/Git commands, migrate configuration or
alter recorded/physical repository state. CLI query arguments continue to supply
explicit author email and retain existing standard-alias isolation.

The registered local-management HTTP capture includes explicit Sync barrier
capture, reached/pending waits, invalid targets and LocalTrusted-only admission.
These browser controls preserve the existing CLI finite-frontier semantics.

The local server composes native Storage management only for LocalTrusted
administrators. A bounded private journal under the configured process state
directory persists captures and reports with atomic owner-only writes and fsync.
GET does not create it. Server-owned work survives request cancellation; after
restart checks report interrupted and checkpoints resolve their original target
from selected authority. No new CLI executable installation or service
registration is required.

Foreground external native serving transfers the exact process state-lock file
into authoritative-storage ownership before capture can retain it. Prepared
server drop and run-future cancellation retain the lock until the final capture
storage lifetime releases; completed ordinary shutdown releases it after storage
stop. Late or duplicate lock attachment fails closed. Filesystem-only serving
retains its independent process lock and remains unavailable for native capture.
Configured Codex startup retains this ownership through ordered capture drain.

Foreground capture uses a prepared slot with disabled, unavailable and ready
states. Disabled selection performs no capture environment/state I/O. Enabled
resource selection validates profiles and privacy locations. Whole-server
preparation, after owning native storage and the state lock, exclusively creates
an absent private capture root with control and import journals and syncs it.
Existing roots and internal resource-only preparation never repair or initialize
missing policy. It then opens control state before installation-bound sources
and the sanitized spool. The ready slot pins control generation and physical
directory/lock plus installation identity, revalidates them at runtime handoff,
and applies the latest selected pause policy before the runtime can dispatch.
Scanner creation waits for admitted body sanitization. Unavailable capture
preparation preserves ordinary serving. The exact ready slot starts with
retained storage ownership and drains before storage shutdown. The ordinary
configured foreground path supports Codex CLI/Desktop; unsupported readers
retain their startup guard.

Settled queued discovery/delivery task loss closes that owner, records one
unavailable observation and invalidates delivery queue freshness. Pause/resume
cannot revive a lost global lane. Explicit replacement must reverify current
admitted source and Write authority and preserve exact pending bytes/cursors.
This adds an internal diagnostic qualification. Typed installation management
uses the qualified retained runtime; configured enablement and real
storage/harness fault gates remain pending.
