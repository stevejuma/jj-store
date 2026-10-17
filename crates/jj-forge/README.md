# jj-forge

Forge adapters, guarded stack submission and recoverable external-effect
journals for GitHub, GitLab and Forgejo. Repository code and operation
identities remain owned by JJ; forge actions retain their own request recovery.

Start with the `client`, `planner`, `engine` and `journal` modules when
composing a forge integration. Plan an explicit stack operation, execute it
under host admission and retain the original journal identity across unknown
outcomes. Saved account credentials use native credential storage.

See the [Git and forge guide](../../docs/site/guides/git.md) and
[transport semantics](../../docs/site/reference/transport.md).
