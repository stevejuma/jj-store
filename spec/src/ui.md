# Storybook presentation catalog

This slice reopens presentation work only. Production integration and deployment
acceptance remain deferred in M4.3. Catalog components receive presentation data
and callbacks; deterministic fixtures own interaction state. Existing browser
entrypoints and API acceptance remain separate.

## Requirements

| ID         | Requirement                                                                                                                                                                                                            |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-UI-001 | The catalog MUST build and run independently using registry dependencies, deterministic fixtures, isolated story state, runtime diagnostics and blocking interaction/accessibility checks.                             |
| JST-UI-002 | The shell MUST directly adapt Community shell/navigation composition and preserve desktop/mobile, sidebar, header, focus and navigation mechanics using public Design Core exports.                                    |
| JST-UI-003 | Branches, JJ Log and Op Log MUST reuse Community graph/row/file/diff compositions, preserve native identities and conflicting targets, and support fixture-only drill-down without transport or mutation capabilities. |

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
