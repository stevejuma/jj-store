# Server selection

## Requirements

| ID         | Requirement                                                                                                                                                                                                                         |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-UI-022 | The server switcher MUST scope application actions, history, drafts, media, notifications and freshness to the selected connection and principal. Inactive responses cannot mutate the selected view or redirect a pending command. |

JST-UI-022 uses a bottom-aligned server list immediately above Help, including
while Settings is open. Server buttons and Add server use the same rounded
navigation button presentation, hover and selected colour as the sidebar's other
destinations. Initials/icons retain accessible names and tooltips; expanded
navigation includes names. Show at most two server entries, counting Local:
Local stays visible and the selected remote occupies the second slot, or the
first saved remote when Local is selected. Additional servers appear in an
accessible horizontal-kebab More servers menu. Selecting an overflow server
promotes it to the visible slot without changing switching guards. Retain
management for the selected remote in expanded and compact navigation. Add
server remains the final connection action.

The application owns connection management, using maintained Community
navigation/form compositions and released Design Core controls. Add, rename,
sign out and remove preserve keyboard/focus behavior and existing form guidance.
The connection dialog follows JST-UI-016: one compact section card, a tinted
heading containing the connection and session-token description, full-width
divided label/help-left and control-right rows, and a separated bottom-right
action footer with Connect last. Narrow windows stack rows and wrap commands.
The dialog uses a compact header with a four-pixel trailing inset and four-pixel
top body padding so the card's outer border clears the scroll viewport edge.
Validation, busy/disabled state, immutable edited origins, session-only tokens
and existing connection actions remain unchanged. Appearance and native window
preferences are device-scoped. Repository, project, agent, profile, emoji and
supported server Settings actions target selection. Unsupported local terminals,
folder picking and machine management are hidden. Remote Settings navigation
contains Profile, Custom emoji and device Appearance; Local service, Agents
administration and Compute remain LocalTrusted management. Default remote
Settings entry opens Profile, preserving explicit section links.

On an authenticated server, Agents exposes a separate Manage server access
dialog only after operator capability discovery. Its configuration and sharing
forms follow JST-UI-016, with retained per-connection drafts, an enrolled
principal picker and one save footer. Existing execution controls remain
caller-scoped. Execution modes use exact IDs advertised by the installed
harness; new Codex definitions seed `read-only`, without inventing a generic
cross-harness mode alias. Access onboarding exposes identity
enrollment/revocation only for `manage-identities` and repository Write grants
only for current repository Admin, using the same principal and explicit
memory-only credential. It explains independent grants and pre-existing native
WorkStore/code requirements. Hidden views pause reads; stale responses cannot
replace selected-server state. Unknown saves/enrollments/grants block switching
and keep their exact recovery intent.

The client-only server query field participates in history and restart. Existing
unqualified links mean Local; shared links use the canonical remote origin and
omit the device connection ID. Each connection retains its destination and
in-memory drafts. Editing and unknown-command guards govern switching/removal.
No secret or protected response is persisted in layout preferences.
Reauthentication preserves an explicit Agents or installation Threads
destination after authorized repository discovery, including reload; it does not
replace that destination with Projects or an automatically selected repository.

The selected connection owns controllers and an authorization generation.
Inactive readers and streams pause; late responses remain isolated. Pending
commands retain their original transport and request identity. Sign-out and
credential replacement clear protected caches and activity. Switching never
stops the local service or remote workers. Initial missing data uses skeletons;
background refresh retains content and drafts.
