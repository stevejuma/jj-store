# Interactive agent execution

Typed installation-local Agents APIs expose runners, targets, conditional saved
setups, session preparation, prompts, permissions, cancel/end/resume, workspace
cleanup, bounded activity pages and request status. Every endpoint requires
LocalTrusted Admin admission; repository commands independently revalidate
source access and physical bindings. Request bodies accept server-issued targets
and exact operation/revision IDs, not executable configuration. SSE/live
reconnect is separate from committed repository freshness.

Status `revision` is the conditional saved-setup revision. Session activity,
publication and process shutdown do not change it; a setup save or removal does.
Duplicate original requests retain their receipt without advancing it. A
concurrent setup edit rejects the stale precondition and retains the browser
draft. Older private execution journals seed the independent setup revision from
their retained installation revision when opened.

Status session views expose optional original setup_id and a read-only connected
boolean. Legacy journals without setup_id load as unassigned; editing/deleting a
setup does not alter existing session snapshots. Connected is projected from a
live supervised process and completed handshake, never accepted from a browser
mutation or reconstructed from persisted idle/running history.

Optional read-only last_activity_at_millis records the device's observation time
of the latest retained execution event. It persists across restart, defaults to
unknown in legacy journals and never advances from status reads or setup saves.
Wall-clock rollback cannot decrease the retained time. No browser command
supplies or edits this field.

Setup description defaults to empty for legacy requests/journals and is bounded
to 280 characters. Avatar remains a string with bounded image/emoji forms. Only
setup writes accept a 2 MiB body; other execution bodies retain 512 KiB.

Each execution event optionally retains observed_at_millis, the device
wall-clock observation time captured when that event is appended. It is
read-only, survives journal restart/resume and publication, and defaults to
unknown for legacy events. It is not a provider-authored timestamp and does not
change native AgentStore identities or authority. Refresh, status reads and
rendering never fabricate missing times.

GET /v1/local/agents/feedback?session=<execution-id> is a bounded read-only,
LocalTrusted Admin projection of repository-selected message targets and
ordinary ThreadStore replies. It maps request/role rows through the owned
capture normalizer, validates exact message membership and captures Agent/Thread
roots in one authorized selection. It does not expose staged objects or grant
linked repository access. The response includes row_id, typed target, discussion
thread IDs and exact visible human-message revisions. Reads never create a
thread, initialize collaboration domains, apply preferences or dispatch a
prompt. Typed repository Thread creation adds create_agent_discussion with an
exact AgentMessage target. Existing collaboration commands, actors, observed
reaction slots, original request/status receipts and admission limits remain
authoritative.

Optional write_unavailable explains the current inbox write capability without
hiding readable targets or replies. Native code repositories with an existing
origin ledger and no WorkStore home retain the reducer-3 publication refusal.
The captured profile selects native or registered-folder inbox transport. Local
collaboration snapshot/query/detail, command/status and retained raster reads
use the separate /v1/local/repos/{repo}/collaboration namespace, current
companion pins, signed query-specific continuations, binding generation and the
existing device actor. Local receipts remain explicitly local; no native
snapshot fields or identifiers are fabricated. Reads recheck current selection
and registration, and query/response budgets fail closed. Raster JSON preserves
validated bounded bytes and media type without fetching external URLs. This
projection never grants write access or weakens command revalidation.

Prompt preparation returns agent_feedback_unavailable (503) when its authorized
repository discussion snapshot cannot be read, or agent_feedback_limit (413)
when the selected new feedback exceeds 128 replies/32 KiB. Both are proven
pre-dispatch refusals: the composer retains the draft, explains recovery and
allows a new explicit Send. No execution intent is saved for these refusals.
Private revision delivery metadata is not exposed as a public session field.
