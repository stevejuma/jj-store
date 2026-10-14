# Retained tree-file progress

Physical installation uses exact retained Git blobs and provider-owned canonical
progress under the existing foreground lease. Prepared file/directory
identities, synced prior intent and no-replace moves preserve originals as
backups. Retry compares the entire namespace and admits only restricted known
transitions. No upstream working-copy loader is invoked and native metadata
stays original.

Native tree completion separately stages the pinned upstream protobufs and
retains a provider-owned plan/inventory-bound completion frame before selecting
tree-state and then checkout. Native RepoPath ordering and signed millisecond
timestamps match JJ; unaffected file states stay exact and Watchman clock is
cleared for the next scan. Exact prepared inode identities limit normalization
during interrupted recovery; complete physical namespace and backup proofs
remain required. This provider completion does not select combination metadata
or acknowledge an application command.

E-CI-BOOTSTRAP-001 retains the original native/local metadata stamp and lock
identity representation while making Unix typedef widening portable. Signed
values keep their existing u64 bit pattern; GNU/Linux and macOS retain the same
field order, inode checks and installation/recovery guards. No upstream JJ
object, operation, view or owned recovery format changes.
