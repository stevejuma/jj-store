# Bounded specification and evidence

E-UI-CI-001 moves the existing core UI acceptance inventory into affected
standard checks under JST-ARCH-005. Local and standalone UI results retain their
source, platform, fixture and failure scope; they do not establish complete
qualification or resolve the recorded backend and cold-application failures.
Generated Changesets and Dependabot governance exemptions still retain every
planned standard check, including core UI.

E-CI-SUITES-001 retains JST-GOV-003/004/011: independent suite results and
reused build artifacts never establish a complete qualification pass. Evidence
records source and producer run/attempt, unavailable timing fields, failed or
canceled tests and remaining gates. The temporary direct-main exception is
explicitly user-authorized and governed by JST-ARCH-005; ordinary protection
returns immediately after each push attempt.

JST-GOV-010 governs all authored specification Markdown, including navigation,
progress and evidence detail pages: at most 400 physical lines and 32,768 UTF-8
bytes. Counts include examples, frontmatter and tables. Generated book staging
and immutable original archives are not authored specification pages. Current
contracts define each requirement once; verification files validate one row per
requirement across their union. Historical source and evidence remain lossless.

JST-GOV-011 permits an explicit unchanged-contract confirmation for protected
refactors and tests. It names affected requirements and source paths, explains
why the contract is unchanged, includes actual validation and confirms the
milestone review. Behavior changes update their owning topics. Every affected
owner requires coverage. A reused parent record cannot confirm a new change.
Task histories derive backlinks from records; task completion still selects
original-gate evidence explicitly. Resolution links never overwrite historical
results or automatically promote milestones. All selected-revision reads use
that revision, and merge checks audit every parent.

The one generated book also bundles tracked repository files referenced by its
pages. Supplements identify their original paths and remain references; they
define no canonical requirements or task states. Transitive links close within
the generated output, source commands and authored files remain intact, and
untracked files cannot enter the book. The folded navigation opens current
reading areas before history and previous chapter links.

## Authoring evidence

Start a record with
`pnpm evidence:new -- --id E-AREA-NNN --task M... --requirement JST-AREA-NNN`;
repeat task and requirement flags for all affected owners. IDs are unique across
all dates. The generated record belongs under `spec/src/history/YYYY/MM/DD/ID`
and remains pending until its actual validation and reviews are filled in.

Retain the `evidence-field` and run delimiters when editing. The schema-1
frontmatter identifies each fragment; prose remains readable Markdown, commands
use shell fences, and run results and structured review fields use YAML. Detail
pages preserve field and run order. The parser restores literal prose entities
and exact multiline commands, including failed attempts. Use an appended record
to correct committed evidence.

After changing records or task rows, run `pnpm progress:history` to regenerate
task backlinks and bounded navigation, then `pnpm check:progress` and
`pnpm spec:first`. Completion evidence is selected in the task table; a passing
history record alone does not complete a task. `spec/history-relations.yml`
records explicit resolution or supersession with a reason while retaining each
original result.

Navigation regeneration paginates wide indexes and writes bounded SUMMARY
fragments under reserved SUMMARY folders. These derived pages preserve complete
chapter coverage and remain outside default search. Generated footer markers are
separated from evidence fields, so headings and delimiters inside recorded
commands remain exact.

## Appended metadata corrections

Committed evidence payloads remain immutable. An appended, verified record may
include `corrections`: target evidence ID, a reason and explicit field changes
with exact `before` and `after` values. The metadata validator applies this
projection only to malformed tests, legacy run metadata and invalid result
vocabulary. It validates replacement values normally and rejects unknown fields,
duplicate corrections, missing targets, correction chains and prior-value drift.

Named test references remain intact. Every command, note and original run
outcome stays exact; valid run results cannot change. An invalid overall result
may become pending, blocked or fail, never pass. Gates, task histories and
immutability audits use the original records, so metadata repair cannot promote
a milestone or make an old failure successful. Record legitimately removed files
through `removed_files`; do not recreate retired dependency overrides. Link the
appended record through history relations with a reason.

Lossless repagination may change an evidence record's page representation after
round-trip equality is proved. Preserve the parsed payload, field/run order,
exact commands and archived sources. Regenerate bounded history/navigation and
qualify the resulting book; do not truncate an oversized historical record.

The migration regression selects the original migration revision for its exact
task/gate and requirement comparisons. Every read uses that selected revision.
Later governed updates remain checked by the current ledger, specification and
immutable archive validators; migration evidence never freezes current product
requirements or implementation progress.

E-CI-RUNTIME-TOOLS-001 retains JST-GOV-003/004/011 unchanged: provisioning a
missing CI tool and preserving structured storage reports do not waive producer,
coverage, evidence or publication gates. Hosted failures remain qualification
gaps even when the workflow and reporting regression tests pass locally.

Qualification measurement evidence distinguishes complete-run timings from
in-job snapshots, scheduler waiting from unavailable runner-only queueing, and
artifact reuse from suites that still compile targets. TAP, libtest and browser
counts retain failures and ignored tests; missing output never means zero tests
or a pass. Comparing workflow layouts requires the same source, inventory and
completed coverage. An earlier canceled or producer-failed run is not a complete
performance baseline.

E-UI-TAG-LIST-001 retains JST-GOV-008 ownership: released Design Core 0.5.0 owns
TagList presentation. Existing Community-derived compositions keep their pinned
Apache-2.0 attribution; JJ labels and state mapping remain application-owned.
