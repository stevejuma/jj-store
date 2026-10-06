# Committed event subscriptions

SSE at /v1/repos/{repo}/events emits only manifest-selected publications, in
publication order. V1 event IDs bind repository, epoch and committed position;
resume accepts after or Last-Event-ID, rejecting contradictory values. A new
subscription starts at the current position with a ready event and snapshot.
Replay never uses candidate filenames, bucket listing or native operation order.
Checkpoint-only manifest revisions do not emit repository-change events.

A subscription checks current read permission and epoch before each delivered
record and heartbeat. Local publication notifications are bounded wake hints;
periodic manifest refresh also observes independent writers. Neither missing nor
lagged hints can skip committed records. Expired replay windows, changed epochs
and positions ahead of authority produce explicit reset/bootstrap events. V1
retains committed history across checkpoints; the bounded SSE replay window does
not delete that history. Resume IDs survive server restart and confer no access.

Subscriptions have separate bounded transport slots, a finite response lifetime,
bounded replay pages/window and storage-read deadlines. Consumers that stop
polling cannot retain resources indefinitely or block publication. Errors after
headers terminate the stream; clients resume with the last fully parsed ID. The
browser uses authenticated fetch streaming because credentials stay in memory.
