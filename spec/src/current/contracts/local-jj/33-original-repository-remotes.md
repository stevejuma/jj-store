# Original repository remotes

JST-LOCAL-008 admits manual remote operations for an existing Git-backed JJ
workspace through a separate native mutation capability. Existing immutable
history-reader grants MUST NOT authorize this writer. Discovery reads existing
remotes/bookmarks without initialization or configuration changes. Adding or
editing a remote requires explicit save and exact prior configuration guards; no
save or connection test publishes data or enables automatic synchronization.

Network work uses a dedicated installation-private transport binding outside the
vault and distinct from managed backup. Repository-provided Git/SSH
configuration MUST NOT execute or select credential endpoints. Native object
import verifies complete bounded SHA-1 closures and retains original bytes and
JJ identities. No network credential enters original repository configuration or
logs.

Manual fetch imports only the selected remote branch with native JJ tracking
reconciliation. It MUST NOT snapshot files, check out content or move a UI
history pin. An operation or endpoint change during transfer refuses the stale
publication rather than reconciling source operation heads implicitly. Existing
unrelated bookmarks, remote configuration and working-copy files are preserved.

Push reviews pin one local bookmark and immutable commit, exact endpoint/branch,
configuration revision and observed remote head. Unresolved conflicts and
non-fast-forward proposals refuse publication. A stable native intent precedes
network IO; a lost response retains that intent. Observation never sends a push,
and another publication is blocked until reconciliation or explicit local stop.
The original workspace's unsnapshotted files never enter a review or transfer.

HTTPS credentials and host-managed SSH use the same admitted transport policy as
backup, with separate private state and credential scope. Host key checking
stays strict. Source library tests, real transport tests and cold host
acceptance are separate gates; M34.9 remains in progress until the latter
qualifies.

Explicit original remote save checks the complete local Git configuration
revision while holding its physical configuration transaction. Both fetch and
push URLs are deliberately saved to the chosen endpoint; inherited push-only
overrides must never redirect an unreviewed publication. Unrelated remotes
survive unchanged. The opt-in real SSH runner exercises original
fetch/publication and backup transport separately, using disposable private
fixture homes with StrictModes.

E-ORIGINAL-REMOTE-002 qualifies real HTTPS using an isolated TLS smart-HTTP
endpoint and disposable container-only trust. Token and anonymous transfers,
authentication refusal, replacement, redirect isolation and exact original
publication retain the same native contract; no TLS verification bypass exists.
The original feature test binary follows the vault-backup profile naming audit.

Native graph counts are operation-pinned and bounded. A known empty remote uses
the native root as its comparison boundary, excluding that root from outgoing
counts; missing local history stays unknown. Original counts read the recorded
remote bookmark without network access. Push-review counts pin the exact
retained commit and observed destination, surviving a later backup capture
without drift.
