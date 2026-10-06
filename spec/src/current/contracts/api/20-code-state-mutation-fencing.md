# Code-state mutation fencing

Head and read snapshots expose `code_state_version` separately from the global
publication sequence and manifest revision. Typed mutation requests and
conflict-upload bindings return that token with the captured epoch and heads.
Agent, review, settings and checkpoint publications may advance repository
position without invalidating a prepared code command; any native code-state
change advances the token and makes the command stale. Generated OpenAPI and
TypeScript contracts carry the field without weakening existing exact snapshot
validation.

Workspace selectors consume the existing native reference windows at explicit
operations, preserving each exact logical workspace name even when several names
share a commit. Browser workspace context is separate from commit/change and
bookmark inspection and is passed to native log queries. Complete workspace
discovery follows pinned reference continuations; it never publishes, reconciles
heads or reads client working-copy files. No HTTP or generated DTO changes are
required for this presentation.

Repository tab labels and right-rail navigation are presentation-only. Overview
retains the existing `files` route, Commits retains `log`, and all seven
destinations preserve their operation and workspace context through existing
controller actions. Responsive overflow neither changes the route grammar nor
publishes native state.
