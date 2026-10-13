# WorkStore service and transports

WorkStore HTTP exposes bounded project/task/comment pages and typed command
apply/status under /v1/repos/{repo}/work. Continuations retain one selected
WorkStore root, repeat the query and page size, reauthorize each page, and reset
after expiry or serving restart. The browser keeps code pins separate and
refreshes task metadata automatically after saves and new data.

E-CREATE-FORMS-001 confirms unchanged WorkStore and native creation contracts.
Settings-style creation cards and the project dialog footer retain existing
create/update commands, title/body fields, writable-home filtering, expected
versions, draft owners and original unknown request IDs. Footer submission uses
the same form/controller; no endpoint, receipt, persistence or authority change
is introduced.

WorkStore browser reads use an independent pinned root per paged collection. The
page-header filter controls reuse the existing Projects controller filter action
and native route values. Moving their presentation from the details rail adds no
requests or contracts; search submission and field selection retain the existing
bounded snapshot reads and automatic browser refresh semantics. The Projects
destination does not require a selected code operation. Stable project/task
route selections reload automatically after live events and bounded periodic
checks. A saved receipt immediately reloads the WorkStore view; ambiguous
outcomes keep the complete request, including exact version preconditions and
entity IDs. Status and retry share that identity, even after refresh. Linked
repository navigation uses the normal target session and token checks, never the
home metadata grant.

Recoverable Projects collection reads keep their independent snapshots and
available rows while the browser reports the original failure through a scoped
application toast. Retry reuses the existing refresh/read owner; moving error
presentation introduces no endpoint, pagination, permission or command changes.

Project list rows reserve aligned title, description, count, creator-avatar and
date columns. Card/list selection always retains one layout. Agent row metric
placement and tool toolbar bounds change presentation only; native records,
snapshots and clipboard values retain their existing contracts.

Project list rows reserve aligned creator-avatar and date columns regardless of
count or date-text length, including narrow lists. They retain the existing
native actor, count and timestamp values.

Task-detail presentation keeps its header and composer outside the content
scroller. Moving the composer changes no comment command, version precondition,
permission or draft owner; Activity and reply/edit actions keep the same native
records and retained draft state.

The development seed exercises these same commands for linked sample projects,
all task states, subtasks and threaded comments. Its cold browser gate reads the
physically reopened native records, verifies zero publication on additive
reseed, and opens the seeded task discussion through the production hub.

The shared browser history independently restores Projects task identities and
the JJ Log evolution anchor. Page changes clear incompatible selections while
preserving the full code operation pin; Back, Forward and reload restore the
selected destination without publishing repository state.

Cold Git bridge browser acceptance uses actual LocalTrusted RustFS publication
and an external merge imported by the custom CLI. SSE automatically reloads the
active current-head graph and operation route while retaining selected commits.
The refreshed graph exposes the imported merge without synthetic agent history.
The same cold fixture verifies the native search adapter catches up to that
import.
