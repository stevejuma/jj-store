# Direct local Stacks CLI

The standalone CLI consumes the ordinary-local Stacks engine through a validated
physical workspace context. Native single-revision resolution, topology,
selected files/hunks and mutation guards are shared with the browser provider.
Read-only inspection and previews load a single existing operation without head
reconciliation, snapshots or settings creation. Execution uses the existing
working-copy/configuration locks, native transactions and descendant rebasing;
no command subprocess, new graph reducer or durable staging state is introduced.

The shared forge orchestration adds saved GitHub account resolution and remote
detection without changing this native code authority. Detection loads existing
remote facts without snapshotting or operation-head reconciliation, recognizes
GitHub.com and explicitly approved Enterprise endpoints, and verifies repository
access before binding. One verified match uses the provider's observed default
branch, including `trunk`; account/remote/fork ambiguity remains explicit. App
requests use their selected account, while CLI automation may supply an explicit
credential. Account switches cannot replace pending external intents, and
submission/Restack validate the account pinned by their original preview. Native
JJ operations, local transaction guards and explicit Restack semantics remain
unchanged; native-storage submission stays deferred.
