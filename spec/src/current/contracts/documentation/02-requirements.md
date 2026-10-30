# Requirements

The ordinary local Stacks manual MUST describe qualified jj-lib bookmark,
selected-file/native-hunk, graph, conflict and repository-wide Undo/Redo
operations separately from the historic retained executor. It documents native
descendant propagation, retained WIP and merge identities, TOML outside history,
explicit membership reconciliation, advisory previews, unknown-outcome
inspection and unsupported partial/conflict shapes. Generated API references and
local, HTTP, contract and application reviews cover these delivered slices.

| ID          | Requirement                                                                                                                                                                                                                                                                                                                                                                             |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-DOC-001 | The application MUST provide a Markdown-first manual with getting-started instructions, concepts, workflows, configuration, application-owned CLI/API references and architecture diagrams. Implemented functionality with incomplete qualification MUST state its limits; upstream-only behavior and unimplemented roadmap features MUST NOT be presented as application capabilities. |
| JST-DOC-002 | CLI references MUST derive from registered command definitions, configuration examples MUST use production validation, and mapped source changes MUST invalidate explicit documentation review acknowledgements. Generated reference comparison, link/navigation validation and diagram validation MUST fail closed.                                                                    |
| JST-DOC-003 | Browser and desktop builds MUST bundle the same self-contained generated manual. Help MUST open from installation navigation in a full-viewport accessible overlay, preserve the underlying view and drafts, and support keyboard dismissal and focus restoration. Help requires no repository selection or internet connection.                                                        |
| JST-DOC-004 | Documentation assets MUST use an independent bounded manifest under the configured browser root. Serving MUST reject unsafe paths, symlinks, missing and duplicate assets; only Help documents may permit same-origin framing. Application and API admission remain independent.                                                                                                        |

E-CONVERSATIONS-NPM-001 applies JST-DOC-002 source review to the public client
manifest, scoped release workflow and all owning artifact/plan/provenance
modules. The technical manual describes complete qualification, exact artifact
and signer identity, owner bootstrap and actual registry verification before its
fingerprint is acknowledged. M30 retains release status authority; reviewed
source and archive evidence do not establish public package availability.
