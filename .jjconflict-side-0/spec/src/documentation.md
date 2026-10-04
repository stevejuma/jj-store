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

UI development MUST validate the generated manual before stopping any running
service. A missing or invalid bundle requires `pnpm docs:build` and leaves
existing processes and runtime markers intact. Vite observes publication of the
documentation manifest through filesystem events, validates a complete new
bundle, and replaces its in-memory assets only after validation succeeds. An
unfinished or rejected rebuild retains the last validated bundle. Missing
initial assets recover when a valid manifest is published. Reloads are serial,
superseded reads cannot replace newer assets, and shutdown releases observers.
This development mechanism performs no periodic reads or application reloads;
production and desktop retain their startup-loaded immutable bundles.

The source review gate records content fingerprints and a topic-specific review
explanation. A passing gate proves acknowledged source coverage, not prose
correctness. The progress ledger remains the sole qualification record. Main
integration regenerates references from the merged parser and API schema, and
reviews newer capture, conversion and desktop terminal behavior before
acknowledging source fingerprints. Workspace retirement follows merged consumer
validation, preserved evidence and process ownership checks. Presentation-only
Help changes may retain accurate manual prose after explicit review; sidebar
sizing follows the application navigation requirements.

Stacks contract and read-lifecycle delivery reviews service/application coverage
against the owning source. The manual distinguishes validated command records
from executable stack operations and keeps recoverable checkout qualification
explicit. Read cancellation and exact-target cache invalidation retain shared
read ownership without changing repository authority.

Bounded stack-history delivery regenerates the API reference from the executable
local query inventory and reviews the local/HTTP/application/contract sources.
The manual describes exact head/base reads and opaque continuations without
presenting them as stack execution or physical working-copy snapshots.

Bounded-read integration regenerates the combined API reference and reviews
source coverage after rebasing. The manual describes files-first enrichment,
independent reference windows and explicit operation-bound preparation/cancel
recovery, while keeping unqualified historical/native/collaboration projections
and large-scale memory/fault gates explicit. Cache admission estimates are not
process-RSS qualification. Read optimizations do not weaken publication or
companion selection authority.

Embedded Help MUST inherit the active application's resolved palette, typography
and chrome preferences on open, page navigation and live appearance changes. Its
Markdown site retains navigation, content and table-of-contents regions styled
with the application tokens. Theme state is read-only and never creates another
application preference record. Standalone documentation defaults to the Original
palette, with its light/dark control selecting Original or Original Dark. Theme
CSS is bundled from the owning application sources; fonts and light/dark diagram
variants remain local and manifest-validated.

The table-of-contents rail has no separating border. At desktop widths the idle
search field aligns with the rail's content edge and uses the app's muted
background and placeholder styling; focused results retain room to expand and
mobile search remains responsive.

Both navigation rails use their full allotted width. The table-of-contents rail
has no extra left inset. Navigation groups start collapsed on the landing page;
readers can expand them, and deeper pages reveal their active navigation path.
The left rail omits the duplicate site title and uses compact navigation rows.
Navigation rows have visible hover and keyboard-focus feedback. TOC links retain
a small text inset and a small gap between rows within the unpadded rail.

Verification covers generators and negative drift fixtures, actual configuration
parsing, built-site links/search, HTTP asset admission, iframe focus and
dismissal, offline cold browser use and desktop resource staging/WKWebView
behavior. Shared asset-delivery changes require all directly affected asset
tests; unrelated repository/storage qualification is not repeated. Visual
baseline approval is separate.

The application freshness manual describes the advertised multiplexed feed,
visible domain interests, hidden dirtiness, retained historical pins/drafts and
exact task/request observation. It distinguishes healthy event delivery from
outage-only fallback and unchanged five-minute renewal from a data refresh.
Canonical ledger evidence owns cold idle/renewal and recovery qualification;
source review acknowledgement alone cannot qualify those runtime gates. Build
and stage the shared generated documentation sequentially within one workspace
so another build cannot replace a manifest's closure during validation.
