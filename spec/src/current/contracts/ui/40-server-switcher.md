# Server selection

## Requirements

| ID         | Requirement                                                                                                                                                                                                                         |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-UI-022 | The server switcher MUST scope application actions, history, drafts, media, notifications and freshness to the selected connection and principal. Inactive responses cannot mutate the selected view or redirect a pending command. |

JST-UI-022 uses a bottom-aligned server list immediately above Help, including
while Settings is open. Rounded-square initials/avatar buttons retain accessible
names and tooltips, an accent selected marker and explicit
offline/authentication state. Expanded navigation includes names. Add server is
the final action.

The application owns connection management, using maintained Community
navigation/form compositions and released Design Core controls. Add, rename,
sign out and remove preserve keyboard/focus behavior and existing form guidance.
Appearance and native window preferences are device-scoped. Repository, project,
agent, profile, emoji and supported server Settings actions target selection.
Unsupported local terminals, folder picking and machine management are hidden.
Remote Settings navigation contains Profile, Custom emoji and device Appearance;
Local service, Agents administration and Compute remain LocalTrusted management.
Default remote Settings entry opens Profile, preserving explicit section links.

The client-only server query field participates in history and restart. Existing
unqualified links mean Local; shared links use the canonical remote origin and
omit the device connection ID. Each connection retains its destination and
in-memory drafts. Editing and unknown-command guards govern switching/removal.
No secret or protected response is persisted in layout preferences.

The selected connection owns controllers and an authorization generation.
Inactive readers and streams pause; late responses remain isolated. Pending
commands retain their original transport and request identity. Sign-out and
credential replacement clear protected caches and activity. Switching never
stops the local service or remote workers. Initial missing data uses skeletons;
background refresh retains content and drafts.
