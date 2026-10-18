# Repository view parity review

Reviewed 2026-10-18. Review complete; implementation open as **NOTES-05I**. The
requested result is jj-store's repository column workflow in Notes, using public
Design Core presentation and Notes-owned native authority.

The previous slices ported the canvas foundation, graph geometry, IDs, file
listing modes and diff renderer. They do not establish complete view parity. The
primary Storybook and production source both still exhibit the gaps below.

## Reviewed sources

- Notes UI baseline: `792cbfa87230561cf0e815ba1cc108d8356a2fcc` in the
  `lapis-desktop-history` checkout. Native continuation in `d00a69b3` changes no
  renderer production code.
- jj-store reference: `5b11966779de92a60876e619c30c5575cef011ac`.
- Actual open preview: port 7490,
  `desktop-repository-history--pinned-history-and-diff`; open Op Log, expand an
  operation, then inspect its details. The review screenshot is retained locally
  at `.local/reusable-conversations/repository-oplog-gaps.png`.
- Design Core's public SearchFilterInput, Command View/Popover, Avatar,
  FileListing, FileChangeStats and FileDiff contracts remain the reuse boundary.
  No package source alias or runtime jj-store UI dependency is proposed.

## Confirmed gaps

| Area                             | jj-store reference                                                             | Current Notes behavior                                                                                            | Required work                                                                                                                                                                                                            |
| -------------------------------- | ------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Search and highlights            | `StoreHistoryQueryHeader`, `StoreHistoryQuery`, `history-language.ts`          | No search toggle/editor in either log                                                                             | Port the header toggle, highlighted native query input, completion, Run/Clear, context and keyboard behavior. Operation fields and JJ revsets are separate languages.                                                    |
| Query execution                  | Operation-pinned `historyQuery` controller                                     | History hardcodes `all()`; Op Log always requests unfiltered operations                                           | Use existing native `query_info`, `graph: log` revsets and `operations_filtered`. Retain applied query/context during paging; invalid drafts retain previous results. No filtering of only the loaded page.              |
| Comparison picker                | `StoreComparisonParentPicker`, `StoreMutationCommitPicker`, graph/target cards | Plain full-ID selects for History parent, operation parent, changed revision and ambiguous before/after endpoints | Compose public Command View/Popover with searchable author/avatar, description, time and exact identifiers. Keep operation IDs, commit IDs and change IDs distinct. Load missing parent metadata by its exact native ID. |
| Parent reachability              | Parent controls remain in the selected-operation surface                       | Operation parent choice is hidden in the separate details column                                                  | Keep explicit merge-parent selection beside the comparison, including narrow panes. No automatic selection of ambiguous endpoints.                                                                                       |
| Op Log details and diff          | `CommunityProjectsRepositoryLogDiff`, `CommunityProjectsRepositoryLogRail`     | A separate Operation details column plus another Diff column                                                      | Stack the collapsible operation card above the selected-file diff in one column. Preserve parent navigation, close/reopen and compact return.                                                                            |
| File and comparison statistics   | FileListing/FileChangeStats driven by actual changed content                   | Counts exist only for the selected file after a content read; changed-path summaries have no line counts          | Supply bounded exact statistics for visible changed files/comparisons. Represent unknown, unavailable and incomplete totals explicitly. Do not turn missing data into zero.                                              |
| Operation message IDs            | `CommunityProjectsRevisionGraphMessage`, `repository-log-message.ts`           | Whole operation description is plain text                                                                         | Port recognition and independent full-ID copying for supported Git/native commit/change IDs, without selecting the operation or nesting buttons.                                                                         |
| Author and revision presentation | Git-author avatar, relative time, typed TagList chips                          | Basic initials badge, absolute timestamp; the adapter drops author email and flattens tags                        | Retain native author data, accessible avatar/fallback and source metadata hierarchy. Use semantic bookmark/workspace/conflict/snapshot chips.                                                                            |
| Graph layout and scrolling       | Per-row occupied-lane inset, both-axis canvas body and virtualized rows        | All rows share the maximum graph inset; default canvas body orientation and a plain bounded list                  | Match per-row content placement and horizontal overflow. Verify forks, narrow nested panes, selected expanded rows, paging and the deliberate rendering bound.                                                           |
| Loading and unavailable states   | Graph/file/diff skeletons and icon Empty compositions                          | Several plain status paragraphs                                                                                   | Port the appropriate skeleton/Empty presentation while retaining typed unavailable reasons, retry and App toast behavior.                                                                                                |
| Parity coverage                  | Production source composition plus governed visual review                      | Existing stories assert the current simplified controls, including native selects and a separate details column   | Replace those expectations with explicit parity regressions; passing existing interactions cannot prove the source design was ported.                                                                                    |

## Data already available

The public `@lapismd/jj-history` contract already exposes `query_info`, filtered
operation graphs, native revsets, exact commit and operation detail reads, and
author name/email/timestamps. Search and rich metadata pickers therefore do not
need a new native search API.

`LocalOperationChangesGroup` supplies change IDs and exact or explicitly
ambiguous commit pairs, not author cards. Resolve metadata through the existing
pinned reader; missing metadata must stay explicit and cannot invent a parent or
pair.

`diff_paths` deliberately returns content as `not_requested`. Notes'
`changeStats` derives line counts from text hunks, and currently receives those
only for the opened file. Correct up-front statistics need a bounded producer or
bounded content hydration with current selection checks. This data step must
precede any claim of complete comparison totals.

## Implementation stages

Only **NOTES-05I** is Active. Its following slices are ordered work, not
simultaneous assignments. NOTES-07 waits while this requested UI alignment is
advanced; its qualified native continuation remains complete.

1. **NOTES-05I1 — Query controls.** Add the exact missing-control regression,
   port highlighted search/completion and native query context, then wire pinned
   filtering, paging, clear and invalid-query retention in both logs.
2. **NOTES-05I2 — Comparison cards.** Replace raw parent/revision selectors with
   public Command View/Popover and source-style metadata cards. Exercise exact
   full-ID selection, unloaded parents, keyboard navigation and ambiguous
   operation pairs. Reuse the presentation for every comparison endpoint.
3. **NOTES-05I3 — Operation composition and rows.** Port stacked operation/diff
   cards, inline message identifier controls, author/time/tags, per-row graph
   insets and both-axis scrolling. Preserve copy feedback and selection fences.
4. **NOTES-05I4 — Exact statistics and state presentation.** Qualify bounded
   native-data statistics, then show them in the comparison/file surfaces. Port
   skeletons and typed Empty states; unknown counts remain explicit.
5. **NOTES-05I5 — Whole-view acceptance.** Exercise both log views, both themes
   and pane widths, forks, merges, metadata refusal, query errors, stale
   replies, restored pins and teardown. Review the primary open stories against
   the named jj-store source surfaces. Visual baseline approval remains
   separate.

## Acceptance and existing work

Already implemented: embedded public AppShell body, Column Canvas navigation and
resizing, pinned native readers, native graph geometry, Git/JJ split ID copy,
expanded folder/list/package FileListing, and public unified/split/wrap
FileDiff. These remain regression targets during the parity work.

Each implementation slice starts with its exact missing/incorrect behavior, then
verifies all directly affected controllers and stories. Changes shared by both
logs require their complete affected interaction/accessibility coverage and the
owning repository's required broader lanes. Native changes qualify their own
library/transport consumers. Record source, browser and data evidence
separately. The 776 previous catalog passes prove the then-current interactions,
not the missing parity requirements above.

The Notes view remains read-only for repository history. jj-store's mutation,
review/discussion and execution capabilities retain their independently scoped
plan stages. This review does not silently add those authorities, close M33's
managed conversation gate or approve visual baselines.
