# Storybook presentation catalog

This slice reopens presentation work only. Production integration and deployment
acceptance remain deferred in M4.3. Catalog components receive presentation data
and callbacks; deterministic fixtures own interaction state. Existing browser
entrypoints and API acceptance remain separate.

## Requirements

| ID         | Requirement                                                                                                                                                                                                                                                       |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-UI-001 | The catalog MUST build and run independently using registry dependencies, deterministic fixtures, isolated story state, runtime diagnostics and blocking interaction/accessibility checks.                                                                        |
| JST-UI-002 | The shell MUST directly adapt Community shell/navigation composition and preserve desktop/mobile, sidebar, header, focus and navigation mechanics using public Design Core exports.                                                                               |
| JST-UI-003 | Branches, JJ Log and Op Log MUST reuse Community graph/row/file/diff compositions, preserve native identities and conflicting targets, and support fixture-only drill-down without transport or mutation capabilities.                                            |
| JST-UI-004 | The complete catalog application MUST expose a typed state/actions controller with independent in-memory navigation, repository snapshots, file contents and profile actions, using copied Community file-browser and footer compositions without backend access. |

The catalog uses Storybook 10.5.7, Svelte CSF 5.1.2, Vitest browser 4.1.11 and
Design Core ^0.2.0 with the existing Svelte 5/Vite 8 toolchain. Direct
adaptations retain provenance and Apache-2.0 attribution. Shared primitives
remain public Design Core imports. Stories are visual-pending until human
review.

## Shell composition

The shell defaults to Branches, a collapsed left sidebar and closed 320px right
sidebar. Per-instance history, selection and controllers never persist across
stories. Repository compact popovers retain focus restoration; mobile navigation
uses the same public shell stages. Community-only room actions are omitted.

## Repository presentation

Branches are JJ bookmarks; conflicted bookmarks show every target and deleted
bookmarks show empty history. A branch, revision and file occupy successive
Column Canvas path levels. Changing an earlier selection clears later selection.
JJ Log preserves divergent same-change commits; only duplicate native IDs may
collapse. Operation siblings remain visible and opening details publishes
nothing. Full-ID clipboard controls never construct Community routes. Source
strings render as data; file/diff controls retain public Design Core semantics.

## Complete in-memory application

The follow-up catalog slice adds Community's repository file tree, branch
picker, source viewer and avatar/profile footer. Choosing a repository opens its
Files view. Bookmarks and native changes resolve to explicit snapshots;
divergent changes and conflicted bookmarks require a target choice. Back/Forward
restores repository, page, snapshot and file selection. Account actions only
change the in-memory fixture identity. Simple shell stories remain alongside
complete application stories; neither is wired to production.

Community's Tailwind Vite plugin compiles the public Design Core stylesheet. The
catalog uses its bundled DM Sans and Source Code Pro fonts, shared icon sizing
and Community typography scale. No substitute font or icon override is
introduced.

Branches use the requested card layout within the existing Command View:
bookmark name, latest description, workspace/conflict chips, author avatar and
deterministic relative time. Filters are All, Workspace and Conflicted,
reflecting native JJ state. The full-app story creates a fresh controller and
has no autoplay actions. The measured graph retains Community paths while
allowing native bookmark labels and ID controls to wrap in narrow columns
without overlapping subsequent rows.
