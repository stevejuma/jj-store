# Query syntax

## History queries

The history query control uses the selected repository's JJ query context. Its
completion metadata is tied to the pinned upstream version. A syntactically
valid expression can still be unavailable for a profile or selection; the
application reports the admitted result instead of issuing an unbounded scan.

Keep the query with its selected bookmark/workspace, operation and time context.
Returning to a saved historical route should preserve that selection. Empty or
incomplete bounded results have different meanings and should not be merged.

The full general language belongs to the
[JJ 0.45.1 revset reference](https://docs.jj-vcs.dev/v0.45.1/revsets/). This
manual documents how jj-store selects and retains that context rather than
copying upstream's entire grammar or promising every upstream function on every
application surface.

## Content search

Content search queries an authorized derived generation. The UI's supported
scope controls select repositories or entity contexts; they are not additional
repository grants. Code source scopes are configured as bookmark/workspace
selections. Collaboration results retain their own canonical entity versions.

Continue a paginated query only with the returned opaque cursor and unchanged
query/context. Expiry, restart or changed snapshot can require restarting the
query. Never alter a cursor or reuse it for a different principal/repository.

## Limits and errors

Queries have explicit resource budgets. A budget refusal or incomplete proof
must remain distinct from a successful empty result. Hidden-hit filtering and
pagination preserve authorization rather than exposing unauthorized counts.
Search freshness and graph preparation are separate: a broken text index need
not prevent a permitted code-history read.
