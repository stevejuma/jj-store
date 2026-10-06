# Capture preview integration planning

At the original review, main's M17 desktop tasks and the capture branch's
historical M17 tasks collided. The integration change MUST retain main's desktop
identity, allocate a free capture family (M18 is reserved by the preview plan
subject to fresh inspection), and record an explicit mapping from historical
capture task IDs. Historical evidence IDs and payloads MUST remain intact; the
merged ledger and its checker must resolve the mapping without attributing
capture evidence to desktop work. Preserve both parent evidence sequences and
deduplicate only identical entries. A rebase MUST exclude the bookmarked
unfinished OpenCode import draft and preserve main's active changes. This
mapping and any checker change require focused governance regressions in the
integration change. The implementation evidence below records the rebase;
completion of the preview still requires its real-service and browser gates.
