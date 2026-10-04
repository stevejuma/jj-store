# Application documentation

The application manual explains current behavior to users, operators and
contributors. The specification continues to own normative requirements and
qualification evidence; internal plans and evidence are not published as Help.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                                                                                                                                             |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-DOC-001 | The application MUST provide a Markdown-first manual with getting-started instructions, concepts, workflows, configuration, application-owned CLI/API references and architecture diagrams. Implemented functionality with incomplete qualification MUST state its limits; upstream-only behavior and unimplemented roadmap features MUST NOT be presented as application capabilities. |
| JST-DOC-002 | CLI references MUST derive from registered command definitions, configuration examples MUST use production validation, and mapped source changes MUST invalidate explicit documentation review acknowledgements. Generated reference comparison, link/navigation validation and diagram validation MUST fail closed.                                                                    |
| JST-DOC-003 | Browser and desktop builds MUST bundle the same self-contained generated manual. Help MUST open from installation navigation in a full-viewport accessible overlay, preserve the underlying view and drafts, and support keyboard dismissal and focus restoration. Help requires no repository selection or internet connection.                                                        |
| JST-DOC-004 | Documentation assets MUST use an independent bounded manifest under the configured browser root. Serving MUST reject unsafe paths, symlinks, missing and duplicate assets; only Help documents may permit same-origin framing. Application and API admission remain independent.                                                                                                        |

## Delivery and validation

The documentation source lives under `docs/site`, shared CLI long help under
`docs/cli`, and maintenance metadata outside the published tree. MkDocs Material
provides the standalone and embedded site. Mermaid sources remain Markdown and
become SVG at build time. Scripts, fonts, search and diagrams are bundled.

The source review gate records content fingerprints and a topic-specific review
explanation. A passing gate proves acknowledged source coverage, not prose
correctness. The progress ledger remains the sole qualification record.
Presentation-only Help changes may retain accurate manual prose after explicit
review; sidebar sizing follows the application navigation requirements.

Embedded Help MUST inherit the active application's resolved palette, typography
and chrome preferences on open, page navigation and live appearance changes. Its
Markdown site retains navigation, content and table-of-contents regions styled
with the application tokens. Theme state is read-only and never creates another
application preference record. Standalone documentation retains its own
light/dark control. Theme CSS is bundled from the owning application sources;
fonts and light/dark diagram variants remain local and manifest-validated.

Verification covers generators and negative drift fixtures, actual configuration
parsing, built-site links/search, HTTP asset admission, iframe focus and
dismissal, offline cold browser use and desktop resource staging/WKWebView
behavior. Shared asset-delivery changes require all directly affected asset
tests; unrelated repository/storage qualification is not repeated. Visual
baseline approval is separate.
