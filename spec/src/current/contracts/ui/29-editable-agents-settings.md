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
