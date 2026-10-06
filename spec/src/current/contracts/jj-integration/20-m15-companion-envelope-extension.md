# M15 companion envelope extension

The owned local ref retains its existing name. Version 2 adds Thread and
Reaction domain selections and typed batch updates. A batch checks every exact
domain selection before preparing one metadata commit/ref CAS and retains a
single original receipt naming all changed selections. Version 1 receipts and
commits remain byte-identical and readable. All selected domain blobs and
retained immutable object blobs have actual Git reachability; hashes in JSON are
not retention. No ordinary Git transport or code operation is implicit.
