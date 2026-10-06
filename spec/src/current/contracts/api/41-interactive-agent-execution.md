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
