# Progressive file blame

The user accepts pinned JJ 0.45.1 annotation semantics and complete-attribution
latency. Native and local Blame use the same `FileAnnotator`, exact operation
pins and authorized prepared core readers. Source is independent of annotation.
Process consecutive topological domains of at most 64 commits and yield after a
cooperative one-second slice. Retain data-only progress and verified summaries;
unresolved lines never receive fabricated or snapshot origins.

Synchronous JJ comparisons are not preemptible. Run annotation on admitted
blocking workers with ordinary on-demand read admission (background metadata
observation must not starve the active view), retain admission until the worker
exits, and check cancellation and deadlines before and after work. Cancellation
prevents further windows and late publication; it may wait for an in-flight
comparison. The original failed E-READ-091 hard comparison deadline probe
remains historical evidence, not a production hard-latency guarantee. No JJ
fork, Git projection, or new index is required by this accepted tradeoff.

Bound cache retention and temporary allocations through shared memory admission.
Continuation retries reuse cumulative progress; source/access checks still run
on cache hits. Continuations bind backend, repository, epoch or registration,
operation, commit, path and algorithm version. Expiry or eviction is an explicit
reset of the same selection. Failed or canceled windows never replace accepted
progress. Core index preparation is allowed; changed-path indexing is not.

The initial adapter admits ordinary UTF-8 files through 256 KiB and 32,768
selected lines, at most 32 pending sources and 16 parents per visited commit.
These are service safeguards, not upstream JJ defaults. Each request processes
up to 64 windows of 64 commits within the cooperative slice, reserving 256 MiB
for trial state/comparison scratch plus separately charged retained state.
Historical blobs use the same byte cap. Existing total read/work/output limits
remain in force. At most eight data-only entries remain cached, with idle LRU
replacement and ten-minute inactivity expiry; valid use renews expiry so a slow
active annotation can finish. Expiry never drops an active worker's charge.
Failed core preparation is terminal for ordinary Blame polling.
