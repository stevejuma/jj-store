# Explicit retained derived-index restoration

Saved-history recovery MAY explicitly restore missing default commit indexes for
the original selected operation and its exact retained candidate. Construct them
from the admitted archive through the existing bounded disk-backed index
builder, without reading source commits, importing extras, reconciling heads or
invoking source index rebuild. Each operation indexes only its own historical
ancestry; the original operation MUST NOT gain visibility of unpublished future
commits.

Before source selection, validate generated hashes, bounded framing, operation
associations and native pinned-upstream decoding in private scratch. Fallback
operation/view traversal MUST be denied during decoding. Preserve existing index
bytes and fail conflicting/unsupported associations without overwrite or
reinitialization. Use held no-follow directory descriptors, synced create-only
segments before associations, the original foreground lease and authorization/
checkout/policy/head witnesses. Admit at most 128 commit segments, 4096
associations, 4 MiB per segment and 16 MiB aggregate; private construction has a
128-MiB scratch allowance and the existing 30-second worker deadline.

Interrupted restoration may leave exact unassociated segments or only one of the
original/candidate associations. Same-request retry verifies those bytes and
completes missing records. Source operation publication, companion updates,
physical checkout and runtime writable capabilities remain separate gates.
