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
