# Search

The application has two distinct query paths: code-history selection and
repository content search. A history query selects code through JJ semantics;
content search uses an independently derived index. An empty result from one
does not establish that an object is absent from the other.

## Search the application

Use the search entry point to choose an authorized repository/context and
inspect matching entities. Results can include supported code and collaboration
content. Follow a result's exact identity rather than rebuilding a route from
its label. Recent items are navigation assistance, not an authorization cache.

Search availability is profile-specific. Native repositories can have a
configured Tantivy index. Filesystem-only serving rejects native adapter
configuration; local indexed search remains separately gated. A disabled
capability should be explained rather than represented as a successful empty
search.

## Freshness

The search worker reads committed repository history and advances a derived
checkpoint. A committed save can therefore appear in repository browsing before
it becomes searchable. Inspect Search settings/status for the worker's selected
position, lag and error state. Search degradation does not make repository
authority unavailable.

A paginated search captures a generation and authorization context. Keep its
cursor with that query. A stale/expired generation requires a new query rather
than merging incompatible pages. Revocation applies even when an old generation
still contains matching content.

## Configure content extraction

Source scopes name bookmarks or workspaces. Include/exclude prefixes and text,
file, work and output budgets bound extraction; exclusions affect only the
index. With no source scopes, retained metadata can still be indexed. Binary,
oversized or unsupported content must not be presented as fully searched.

Changing source configuration requires the documented Apply/restart behavior for
that setting. Rebuilds use a new derived generation. They never rewrite native
code or collaboration roots. See [configuration](../reference/configuration.md)
and [read/search architecture](../technical/reads.md).
