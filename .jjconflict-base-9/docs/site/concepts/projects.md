# Projects and linked code

A project home stores its project, task and discussion records. Code links can
point elsewhere, allowing a coordination repository to describe work across
several code repositories without moving their code or merging their access
policies.

## The home owns the command

A task edit is authorized and published in the metadata home. Reading a linked
file is authorized separately in its code repository. This is why a user may be
able to discuss a task while its code link is unavailable, or inspect code
without permission to edit the project's task state.

Task hierarchy is within the owning project/home policy. Parent changes, state
changes and comment edits use exact versions and stable request IDs. Concurrent
edits are handled through the domain's stale-version rules; the browser does not
overwrite unseen work merely because a title happens to match.

## Choosing a native home

Creating native WorkStore metadata preserves and freezes an existing origin
ledger until a qualified WorkStore reducer can carry those records. A home
therefore has different replication capabilities from a plain native code
repository. A separate metadata home is useful when code repositories need
ongoing native replication.

## Choosing a local home

Git-backed JJ homes use the shared WorkStore model through companion metadata.
Their commands can run without a native service, and workspaces sharing the
repository share selected metadata. The companion ref has its own publication
and receipt history. Normal code undo does not rewind project discussions, and
normal Git network commands do not automatically transfer that history.

In either profile, a local path is a device binding rather than the project's
portable code identity. Use stable repository links when collaborating and
register the appropriate physical folder independently on each device.
