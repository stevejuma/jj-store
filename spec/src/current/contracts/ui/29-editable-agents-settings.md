# Editable Agents settings

JST-UI-012 includes an Agents Configuration tab using the same divided Settings
rows, two-column fields and stacked action column as existing settings. Local
Admin users may edit capture enablement, qualified Codex profile selection,
absolute directory overrides and bounded worker limits. Defaults and currently
configured roots MUST remain visible; saved versus startup-effective state and
restart-required status MUST be explicit. Drafts survive tab navigation and
refresh. Stale writes never silently overwrite another editor; unknown save
outcomes retain the original intent until a fresh observation resolves them.
Operational failures use the shared notification service. Field validation and
restart guidance may remain contextual. Remote and non-Admin connections do not
expose editing. No credential or arbitrary TOML editing is introduced.

Cursor and OpenCode configuration MUST share Codex's rounded harness group,
expandable avatar-bearing rows and divided nested settings. Cursor has
independent Desktop/CLI toggles, root overrides and visible resolved defaults;
OpenCode has a toggle and explicit local API endpoint. Global enablement and
budgets remain outside harness sections. Loading skeletons, draft retention,
notification-service errors, save/cancel columns and restart-required state
apply to all harnesses. Sources and Agents reuse their existing avatars,
repository links and filters. New harnesses with deferred inactive import MUST
keep Import unavailable.

Cursor and OpenCode capture sources reuse the existing Agents presentation,
harness avatars and cached Sources table with repository, harness and status
filters. Current coverage distinguishes withheld inactive, disabled, unsupported
and unlinked sources; absent native message/tool timing remains unknown. Cursor
settings explain the one-time `jj agent setup cursor --transcript-capture` step.
Saved/effective values and restart-required state apply independently to both
Cursor products and the configured OpenCode API endpoint.

Settings → Agents includes a Runtimes tab with the existing divided settings
rows, harness icons, readable tooling status and Check again. LocalTrusted Admin
users may explicitly Install or Update the fixed Codex runtime. Confirmation
names the pinned adapter and CLI versions, private device scope and download.
Readiness means executable tooling, not authentication or agent presence. Cursor
uses its vendor setup/update guide; unsupported installers are never implied.
Missing Node.js 22+ or npm links to Node's setup guide. Refresh retains rows and
reports Unknown on failure; installation state and original request identity
survive tab navigation. No read, refresh or chat action installs tooling.

Ready runtimes use Community's green success chip and omit Install. Guide links
and installation confirmation actions use the shared semantic button styling,
with inline icons. Runtime and capture tabs show only their own controls.

Runtime summaries place the harness avatar in a separate first column and all
identity, status, description and version text in the second column.
Descriptions use Community's smaller secondary text size. Loading rows retain
this alignment; the trailing action controls and runtime management behavior
remain unchanged.

M24 adds
[reusable agent conversations](../collaboration/37-reusable-agent-conversations.md)
under JST-COLLAB-034 through JST-COLLAB-036. Existing M23 qualification remains
unchanged.
