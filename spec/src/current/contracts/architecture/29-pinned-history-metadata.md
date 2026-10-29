# Pinned history metadata

JST-ARCH-007's shared history service exposes bounded query information,
workspace and unfiltered bookmark lookup, and operation detail through an
already ready handle. These point metadata reads use the existing local metadata
reader; they do not require rebuilding a graph index. Caller scope, vault
generation and source admission remain checked at entry and completion.

An operation detail outside the handle's anchor must first prove it is an
ancestor through bounded operation metadata. Reference lookup keeps the existing
outer/source proof. A later source head cannot become visible through the old
handle. Reading metadata neither snapshots working files nor updates source
indexes, operation heads or the retained handle pin. Other query families retain
their qualified prepared readers and explicit capability limits.
