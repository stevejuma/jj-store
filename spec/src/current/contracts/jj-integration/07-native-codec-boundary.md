# Native codec boundary

Collaboration RPC uses owned service/domain contracts for Thread and Reaction
commands and reads; native factories and code topology remain independent. The
admitted peer supplies the actor through the service, and typed receipts retain
the original request across transport reconnection. Raster bytes have their own
bounded binary carrier; neither a cursor nor a content ID authorizes reading an
unselected asset or publishing repository state.

The integration crate owns explicit conversions for all pinned commit, tree,
copy, view, and operation fields. File and symlink IDs hash raw bytes; the other
kinds hash the full native value. Submodule IDs are preserved as opaque external
links. Decoding validates structure before invoking native constructors that
would otherwise panic. Physical integrity alone never establishes native
semantic identity.
