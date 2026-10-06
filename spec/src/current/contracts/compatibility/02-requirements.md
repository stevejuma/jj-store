# Requirements

| ID             | Requirement                                                                                                                                                                                                                                                                  |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-COMPAT-001 | Both JJ crates MUST be pinned to the same tested 0.45.1 release and upstream revision, with licenses, trait coverage, and golden identities recorded before native storage rollout.                                                                                          |
| JST-COMPAT-002 | M0 MUST prove compiled extension/context/query boundaries and real RustFS conditional writes before dependent storage milestones can be completed.                                                                                                                           |
| JST-COMPAT-003 | Local release qualification MUST include persistent graceful and abrupt restart, missing-state and ownership failures, bounded resource measurements, LocalTrusted security, semantic parity, and cold browser/lifecycle evidence.                                           |
| JST-COMPAT-004 | Migration and passive replication MUST remain deferred for the first release; failed future imports MUST leave source repositories untouched and MUST NOT silently remap IDs.                                                                                                |
| JST-COMPAT-005 | Local release qualification MUST pin the embedded RustFS crate/source/features and prove macOS ARM64 cold startup and persistent restart; Linux packaging, external images/providers, generic S3, backup/restore, replication and migration MUST remain explicitly deferred. |

A2 qualification runs the released embedded helper through the actual server and
CLI binaries on macOS ARM64. It verifies tokenless native creation, a mode-0600
Unix socket, protected loopback HTTP, explicit catalog registration, restart and
browser bootstrap. Linux packaging, authenticated/shared deployment, external
S3, backup/restore, generic-S3 qualification, replication and migration are
deferred future-release work. Explicit LocalTrusted search uses the same native
authority and embedded storage without a bearer credential. Registry-only shared
browser integration and package publication are deferred independently. Windows
remains unsupported for LocalTrusted because its required Unix peer boundary is
absent.

Local agent capture pins MongoDB Kingfisher v2.7.0 at source revision
`63dccba231b6b01990c84319d25f92f49428a431` under Apache-2.0. The scanner uses no
default features or Base64 decoding, and Cargo selects Kingfisher's matching
Vectorscan fork at the same revision. Qualification covers bounded in-memory
redaction, fail-closed omission and fresh-process compiled-cache startup; it
does not enable Kingfisher validation or network access.

Capture spool coordination retains the V1 pending record format. New handles
share the former lifetime-exclusive `capture.lock`, excluding old writers while
using separate short mutation and single-publisher drain locks. An older binary
that already owns the spool blocks new handles before queue access. New hooks
and drainers can share state only when both follow the new lock protocol; this
does not upgrade installed binaries or install a background recovery worker.

Automatic readers add optional private `reader_metadata` generation information
to pending V1 entries and a bounded owner-only `reader-progress/` journal.
Legacy enqueue omits that field and retains identical serialized pending bytes.
Older strict readers reject the additive reader field; the compatibility lease
excludes older concurrent writers. Repository objects, RPC requests, prepared
native/local requests and receipt interpretation do not change. Progress is
retained after publication and repaired from sanitized pending evidence before
request removal; the new device journal is neither repository state nor portable
history.

The additive Codex JSONL source distinguishes CLI 0.156.1 and Desktop 0.159.2
through exact header markers. CLI `cli` and `exec` sources accept the qualified
`codex_cli_rs`, `codex_exec` and inherited `Codex Desktop` originators; CLI
originator inheritance does not admit the Desktop `vscode` surface. Unknown
originators and versions remain unavailable. The previously unqualified Desktop
configuration pin of 0.156.1 is rejected; its enum/config field names are
unchanged. Native hook profiles, sanitized object bytes, prepared requests and
receipts retain their prior interpretation. This source-only reader does not
enable the server worker or claim real-harness capture qualification.

Codex location resolution and incremental metadata discovery are additive
read-only runtime capabilities with no durable format or configuration-schema
change. Explicit roots precede CODEX_HOME and default user-home conventions; the
scanner binds physical roots and rejects rebinding/symlink escapes. Neither
capability enables automatic capture or qualifies other harness locations.

The Desktop 0.159.2 normalization profile accepts explicitly bound sanitized
native transcript envelopes and rejects unqualified Desktop hook interpretation.
Existing CLI 0.156.1 hooks retain their prior bytes and meaning. Native message
IDs and source-derived missing-ID identities have distinct namespaces. The
transcript-specific sanitizer policy adds home/temporary and key-path privacy;
the prior hook policy, prepared requests, receipts and repository object formats
are unchanged. Unsupported native records remain retained sanitized evidence
with diagnostics; richer block projection is separately gated. Native function
and custom-tool results use a transient typed resolution boundary without adding
persisted object kinds, event variants, prepared-request fields or wire schemas.
Exact same-source aliases resolve retained calls and preserve their existing
identities and terminal results. Unknown/unmatched outputs remain evidence with
diagnostics; legacy hook interpretation and receipts are unchanged.

Qualified common OpenCode aliases retain the existing V1 object and receipt
formats. New hook/API/export captures may adopt exact case-sensitive selected
session/message/tool IDs and additionally edit `opencode-native-alias-join/v1`
in the existing projection catalog. The Codex common key and its UUID case rules
are unchanged. Previously prepared bytes and prior duplicate identities remain
retained; incomplete or ambiguous selected proof produces a diagnostic rather
than merging existing sessions or changing the interpretation of an old receipt.

Focused authenticated compatibility now covers private registry initialization,
case-sensitive slash-separated aliases, one manual token with independent
repository permissions and revocation, forged-secret rejection, cursor reset,
and memory-only browser selection. It does not qualify an external S3 provider,
public shared listener, OIDC, Linux package or deployment topology.
