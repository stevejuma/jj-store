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

## Qualified source automation

JST-ARCH-005 retains private root/native release synchronization, public
History/Conversations publication, immutable source and platform qualification.
The reviewed Node22 CLI2 complete recipe and source facts precede App admission.
Intent qualification compares the changed candidate to exact current remote
main; a post-preparation same-main no-op is not a changed-candidate proof. With
no note, CLI status rejects changed private workspace packages even when the raw
version macro exits successfully without changing metadata. The measured empty
maintenance note lets the guard report false intent without acquiring App/writer
authority. False release intent skips the App/writer and retains existing
workflow-token orchestration. Successful reviewed PR/push/manual workflows
notify only the pinned coordinator with read metadata and one named App key;
workflow-call-only events skip before invocation. Source acceptance does not
establish activation, publication, hosted callbacks or complete native release
qualification.

<!-- dependency-intake:framework-registry:start -->

Current registry dependency coordinates; qualification remains separate.

<!-- prettier-ignore -->
| Manifest | Dependency | Range | Locked version |
| --- | --- | --- | --- |
| `package.json` (devDependencies) | `@lapismd/design-core` | `^0.13.8` | `0.13.8` |
| `package.json` (devDependencies) | `@lapismd/mira` | `~0.2.0` | `0.2.0` |
| `package.json` (devDependencies) | `@lapismd/spec-validator` | `^0.2.1` | `0.2.1` |
| `package.json` (devDependencies) | `@lapismd/storybook-addon-visual-delta` | `^0.1.5` | `0.1.5` |

Manifest, peer and integrity facts SHA256:
`f872c03c1f4e6ef2937dd39bd968ec6eae60017ab696e382d8bdcd9e4cef0e03`.

<!-- dependency-intake:framework-registry:end -->
