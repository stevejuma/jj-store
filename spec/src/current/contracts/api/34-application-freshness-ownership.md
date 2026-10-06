# Application freshness ownership

Successful Threads refresh accepts its current authority watermark independently
of the retained historical code selection. Fresh snapshot reads invalidate only
the live snapshot key; exact operation-bound content keeps its cache entry.

Repository transports and query caches belong to an authorization session. View
leases share exact native/local reads; each cancellable caller owns an observer,
and only the last active observer release may abort pending transport. Completed
reads release their caller lease, so a later caller abort cannot cancel a
completed or subsequently shared request. Snapshot recapture invalidates only
the live head key; events and confirmed commands invalidate affected
collaboration families. Local fresh captures retain supersession while ordinary
query keys bind actual operation/companion and registration generation.
Immutable object reads retain infinite staleness with bounded inactive
retention; prepared availability, attribution and expiring continuations remain
transient.
