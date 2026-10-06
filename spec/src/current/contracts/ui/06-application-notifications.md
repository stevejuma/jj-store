# Application notifications

JST-UI-019 requires one application-owned, session-only notification service and
public Design Core toast presentation. Condition notices clear on recovery or
when their resource becomes obsolete; acknowledgement notices survive navigation
until dismissal, and routine informational outcomes expire after four seconds.
Repeated observations update one stable scoped key. A dismissed unchanged
condition cannot replay during polling or remount; resolution permits a new
episode. Pointer and keyboard interaction pause timed dismissal.

Repository browsing state feedback, including graph/inventory preparation,
cancellation, incomplete history, conflicted targets and metadata failures, uses
persistent scoped toasts. Cancellation and retry actions live in those toasts;
target-selection controls retain their explicit choices. Index progress does not
add body status rows or replay a dismissed running notice as counts change.
Pending graph, query, directory metadata and paged rows use accessible
skeletons; loaded content remains available during background work.

Field/query validation, content empty states, stale drafts and exact mutation
request recovery remain contextual. Reaction metadata retains its existing
placement. General request failures and confirmations use toasts without
duplicating inline announcements. Actions retain exact command identity and
never change repository state merely because a toast is dismissed.

The application clears protected notification state on session changes and
teardown. New notification records are independent of transport and DOM;
optional subscribers receive plain data for a future selective desktop adapter.
No OS notification delivery, disk persistence or notification-center UI is
introduced. Design Core owns severity icons/colors, keyboard controls, portal
placement and modal focus compatibility. Catalog owners remain isolated from
production state.
