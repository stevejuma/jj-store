# Bookmark publication profile

GB5 qualifies bookmark patterns, revisions, all/tracked/deleted bookmark
selection, change-generated and explicitly named bookmarks, plus atomic
multi-ref transport. This profile publishes bookmarks only; tag publication and
Git push options stay unsupported. A selected unsafe ref rejects the native plan
before external writes. Every update retains its own exact destination lease.
More than one update requires remote atomic capability, with no fallback to
partial publication.

Generated selectors resolve names and targets once, validate the complete plan
and conversion, and then explicitly prepare the local bookmarks in an accepted
native operation before the durable intent. That frozen operation's bookmark
fields provide the same source proof as existing bookmarks. Dry-run publishes
neither generated bookmarks nor an intent. Interrupted publication retains the
prepared local bookmark and durable attempt for explicit recovery.

Remote rename preserves its logical identity, destination fingerprint and causal
facts while atomically moving native tracking metadata. Removal tombstones the
alias and removes only its native tracking view; local bookmarks, immutable
objects, origins and attempts survive. Reusing the name creates a new logical
identity and cannot inherit the removed remote's leases.
