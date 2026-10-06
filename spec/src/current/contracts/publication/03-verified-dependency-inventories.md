# Verified dependency inventories

Inventory construction follows graph edges from requested roots, never a list of
objects uploaded by one process. The native source verifies bytes and identities
before returning dependencies. Operations include parents, their view and every
key/value of commit_predecessors; commits include parents, old predecessors and
all root-tree merge terms; views include all reference terms and workspace
commit mappings. Trees include files, symlinks, subtrees and real copy IDs; copy
histories include ordered parent references. Submodule links and copy
placeholders are not repository-object dependencies. Virtual roots and the
pinned empty tree are represented explicitly and require no persisted blob.

The inventory walker rejects missing/corrupt objects, cycles and mismatched
keys. It enforces limits on total nodes, edges, newly loaded bytes and elapsed
time. A previously verified inventory can supply an incremental base; new
inventories retain its entries and add the verified closure, so evolution-only
objects cannot vanish when visible heads change. Only a later manifest selection
establishes committed authority; a verified inventory by itself does not
establish a commit.

In-memory verified bases are bound to a process-local source capability and
repository identity. A different accessor, replica or restarted process must
recover certification from selected persistent records; it cannot reuse an
arbitrary in-memory base to skip required reads. The source receives the
remaining byte budget before starting a new object read.
