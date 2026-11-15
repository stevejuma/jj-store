# Version PR authentication

JST-ARCH-005 generated Changesets version PRs MUST use the existing dependency
App only on canonical main, scoped to this repository with contents and
pull-request write permissions. Missing credentials MUST fail closed and
checkout MUST NOT retain credentials. The existing release orchestration and
explicit full-check dispatch MUST retain workflow credentials and their
actions-write authority. Changesets MUST NOT create GitHub releases directly.

This authentication boundary changes no immutable-source, signing, native,
container or publication qualification gate. M30 remains in progress. Source
workflow checks cannot establish a successful hosted App-created candidate or
complete release qualification.
