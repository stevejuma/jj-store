# Cursor transcript hook handoff and shared file reader

Explicit `jj agent setup cursor --transcript-capture` installs owned project
hooks. Hook receipt may atomically write a bounded private `*.jj-store.json`
binding beside a canonical regular JSONL transcript under Cursor's approved
project roots. This explicitly opted-in harness metadata is the exception to
memory-only acquisition locations: it contains only the qualified profile and
version, native conversation UUID, exact workspace/transcript paths and current
activity receipt. It MUST contain no prompts, responses, tool arguments,
credentials or repository IDs. Server source/spool catalogs remain digest-only;
this hint is neither server state nor repository authority.

The hint requires exact single-root workspace matching, private permissions,
bounded regular-file reads, physical identity checks and atomic replacement.
Nullable paths or not-yet-created JSONL files retain only an active session
witness under the approved root's private `.jj-store-bindings` directory. This
handoff is bounded to 256 metadata files per root, expires after 24 hours,
prunes expired witnesses and carries no message bodies. A later terminal hook
may transfer that witness only after matching the exact session, product and
workspace and validating its now-available physical transcript. Its recent
terminal receipt grants the bounded flush window; it is never message time.
Completed handoffs remove the pending witness. Non-JSONL paths are withheld.
Hooks in transcript mode MUST NOT enqueue their prompt/response/tool bodies. An
unadmitted source requires a witnessed active hook receipt no older than 60
seconds. Terminal hooks retain that timestamp only from a previously written
exact binding, or complete the matching pending active-session witness, so short
sessions completing between scans can flush. A terminal hook alone or a stale
hint grants no admission. A server-linked native directory, exact workspace
binding and current Write admission are still required before source allocation.
Receipt time is eligibility metadata only, never a fabricated message timestamp.
Changed workspace/profile witnesses cannot inherit prior Cursor admission.

Cursor Desktop 3.22.7 and CLI 2026.09.26-dd393fe use versioned bindings with
separate product qualification and the shared bounded file reader, source
leases, sanitation, durable spool and publication receipts. Cursor has no
first-line metadata header; its header witness binds the explicit hint and
physical file. Complete-prefix verification, partial writes, rewrite generations
and monotonic source sequences retain existing file-reader rules. Source-derived
record identities use exact cumulative complete-prefix digests. They do not
claim native message/tool IDs. Unchanged prefixes retain the same presentation
identities through rewrite recovery. JSONL `text` and `tool_use` parts preserve
available text/tool arguments; missing call results, timing and lifecycle remain
unknown. Explicit `turn_ended` status may record idle. Hooks do not duplicate
transcript messages. Previously admitted exact witnesses follow delayed final
flushes and recover through restart without renewed activity. Cursor
history/import targets remain withheld. Shared source/read/frame budgets,
pause/resume/rescan, separate delivery workers and cached Sources observations
apply to all four file profiles.

Configured LocalTrusted OpenCode API startup now selects the existing retained
read-only supervisor lane under its strict explicit loopback endpoint and native
workspace admission. Endpoint outages degrade that lane and retain original
recovery intents while other harnesses continue. Startup preparation performs no
API request. API history/export, direct database readers and real-product
release qualification remain separate from fixture and startup evidence in the
ledger.

When native API assistant metadata supplies a creation-time object without
completion, the automatic reader MUST stop at that message and reacquire the
prefix after the final flush. It MUST NOT enqueue the mutable partial body as an
immutable intent or skip ahead to later messages. Existing retained intents are
never discarded to repair a changed snapshot.

Agents read projections MUST recover available message creation/completion and
tool start/end times from matched, qualified OpenCode snapshot evidence without
rewriting immutable batches or durable identities. Only native nonnegative
integer times with ordered spans qualify; missing or malformed fields remain
unknown. Hook receipt time and embedded prompt text are not native timestamps.

A bounded shutdown timeout retains its exact owner and leases. Once that owner
proves every lane stopped, retry reports successful drain; an expired earlier
wait MUST NOT become a permanent failure after successful settlement. Genuine
driver faults remain terminal, and storage teardown still waits for owned drain.
