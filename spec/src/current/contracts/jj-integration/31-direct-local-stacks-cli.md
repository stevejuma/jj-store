# Direct local Stacks CLI

The standalone CLI consumes the ordinary-local Stacks engine through a validated
physical workspace context. Native single-revision resolution, topology,
selected files/hunks and mutation guards are shared with the browser provider.
Read-only inspection and previews load a single existing operation without head
reconciliation, snapshots or settings creation. Execution uses the existing
working-copy/configuration locks, native transactions and descendant rebasing;
no command subprocess, new graph reducer or durable staging state is introduced.
