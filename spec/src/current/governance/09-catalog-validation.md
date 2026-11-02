# Catalog validation

E-VISUAL-DELTA-MACOS-001 adds registry Visual Delta 0.1.0, shared capture
settings and an ignored machine-local native selection. Its CLI, Storybook panel
and Playwright suite MUST resolve the same profile, static-server port and
persisted PNG/artifact namespace. CI ignores local overrides. Comparison
thresholds and interaction/accessibility gates remain unchanged; disposable
capture evidence does not approve committed visual baselines. Shared Storybook
registration requires the catalog check, unit, full interaction/accessibility
and cold static browser lanes, plus focused native capture verification. Rust
and live-server lanes are outside this development-tooling surface.

Production presentation source lives under `web/src/lib`; catalog source owns
only fixtures, stories and harnesses. Both areas require the UI chapter and
same-change ledger evidence. Normal and specification lanes include catalog
types, unit tests, Storybook interactions/accessibility and cold static builds.
The catalog may import production presentation modules but its source audit
rejects API clients and network transports. Production cannot import catalog
fixtures. The progress checker recognizes catalog and Storybook paths.

Cold catalog acceptance builds static assets and serves them on an owned
ephemeral loopback port. Browser tests reject API, external HTTP and WebSocket
requests, verify responsive navigation and fresh-instance state, and retain
unapproved screenshots only as local review artifacts. Virtual-row geometry is
compared within each component's directly owned row container and in rendered
vertical order, so nested listings and independent columns cannot produce false
overlap failures. The production web manifest includes only flat, regular
`agent-icons/*.svg` files from the public icon directory; source attribution
documents and all other public files remain outside the served allowlist.

The owned native agent timeline and grouped tool-call presentation adapt
Community's adjacent-call grouping and per-call wrapper composition. Their
source paths, pinned revision and license remain in `docs/ui-provenance.json`;
native IDs, controller state and review actions stay owned by this repository.

Normal checks also run the Community-derived browser architecture audit. It
enforces counted-line limits and exact no-growth budgets for existing hotspots,
rejects production imports from catalog fixtures, and keeps reusable
presentation modules independent of API/controller ownership. Its focused
negative tests run in the governance lane.

LocalTrusted catalog stories exercise bootstrapping, multiple registered
repositories and the empty catalog using the same production component. The
Storybook lane keeps interaction and accessibility failures blocking. Cold
browser fixtures must provide the bootstrap phase explicitly and assert the
protocol header plus the presence or absence of bearer credentials for the
selected access mode. Visual baselines remain a separate approval lane.

The LocalTrusted demo seed is verified at its native boundary: the unresolved
file retains JJ conflict terms, while the divergent history presents distinct
commit IDs under one change ID. Focused real-browser acceptance asserts those
full identities through production controls. Catalog graph units separately
prove stable lane geometry, so native divergence is not inferred from color or
screen position. The final seeded working copy intentionally has an empty native
description so the production fallback is exercised without fixture-only
presentation data.

The tracked pnpm hook may substitute an available sibling Design Core source for
dependency, development-dependency and optional-dependency fields only. Peer
ranges, the published package range and the committed registry lockfile remain
unchanged. Governance tests cover both local-source resolution and the registry
fallback when that sibling is absent; local installation uses `--lockfile=false`
so validation state cannot rewrite the release lock.

Community presentation is copied into this repository. The provenance manifest
covers recognized Community families, explicitly marked copies and extracted
presentation components; native helpers are separately identified with reasons.
The ownership plugin rejects missing coverage as well as every external
Community dependency. The owned StoreWork task detail, Activity/comment
composition and task stylesheet record the CommunityProjects source revision and
Apache-2.0 attribution in this manifest. Their adapted layout retains native
WorkStore commands, identities and drafts without importing Community runtime or
fixture state. A renamed copy outside recognized families must carry an
`@community-copy` source marker so the audit can enforce its provenance. Static
analysis does not claim to identify arbitrarily renamed unmarked copies. The
historical M9.5 package consumption is superseded by this owned-source boundary.
Buzz-derived Appearance provenance lives in
`docs/reference/appearance-provenance.md`; generated palette, Shiki and Inter
redistribution licenses are retained under `docs/licenses` alongside the
Community ownership manifest. Catalog interactions and accessibility remain
blocking; visual baselines remain separately approved.

M4.8 review acceptance covers populated and filtered branch dividers, the
absence of duplicate history-page titles, shared column header styling, and
directory navigation history. It also covers the Files page navigation
breadcrumb, body-left file-tree sidebar padding, full-height divider, bordered
directory table card, commit/action summary row, in-card snapshot selector,
distinct table-header grey and root README, CODE_OF_CONDUCT, CONTRIBUTING,
LICENSE/LICENCE and SECURITY preview tabs. Landing-document acceptance covers
friendly labels, document icons, compact type, the width-measured `More`
dropdown, its hidden-document selected treatment and the selected-document edit
action. It also covers body repository tabs, the moved breadcrumb and
authenticated UTF-8 file edit/rename preview and publication. Selected file
acceptance covers copied metadata, file statistics, filename source headers, the
persistent Preview/Source/Edit header, body-only editing, Save/Cancel and the
description dialog, the borderless line-numbered Mira MDE body with
filename-matched language support, theme-background source bodies,
draft-preserving Markdown preview/source toggling and `.rs` source highlighting
through the public CodeBlock tokenizer prop. Cold acceptance locates the
persistent column controls after the redundant title bars are removed.
Screenshots remain visual-pending and cannot substitute for
interaction/accessibility checks.

The cold catalog lane checks native pointer and touch split-copy behavior,
constant-width geometry and reduced motion against freshly built assets.
Synthetic story hover events alone do not establish CSS hover acceptance.

Split-copy cold acceptance checks four-character resting labels and expanded
five-character prefixes with two dots and a two-character suffix. It prevents
identifier clipping while either section expands. Single-identifier stories
verify that the prefix, dots and suffix are visible immediately at rest.

The default split-copy control stays within 122px width and 27px height at the
standard root scale, including active and independent feedback states, with
balanced inline padding.

M4.16 mutation catalog stories exercise real menu, form, preview, confirmation,
apply, status and close controls against an isolated in-memory controller. The
production browser lane separately proves captured HTTP requests and a cold
native RustFS-backed mutation. Because the application canvas already owns
scrolling, repository menus and dialogs disable redundant body scroll locks and
tests require body pointer/overflow state to be restored before navigation.
Visual comparison is opt-in through the configured capture profile; stories
remain visual-pending until separate baseline review.

Collaboration presentation acceptance keeps focused agent/review Storybook
interactions, the complete 84-story interaction and accessibility suite, a cold
static catalog browser, production controller tests and an owned LocalTrusted
reset distinct in the evidence ledger. The live reset must record that seeded
hook capture currently lacks an exact code-association publication path; catalog
association coverage cannot be presented as live Agent Context evidence.

The [test quarantine index](25-test-quarantines.md) also tracks native cases.

## Temporary story quarantine register

The user explicitly authorized quarantining and disabling flaky tests during the
2026-11-01 audit. Only the individual cases below carry the `quarantined` tag.
The Vitest addon skips that tag and retains each case in its reported inventory.
Other cases in the same files, runtime diagnostics, interactions and
accessibility checks remain enabled. Quarantined coverage is not passing
coverage and cannot complete M30 or replace the original acceptance gates.

Evidence:
[E-LOCAL-TEST-AUDIT-001](../../history/2026/11/01/E-LOCAL-TEST-AUDIT-001/index.md).
The initial remote suite passed 1,046 of 1,048 Storybook cases. Both failures
then passed unchanged in three independent fresh Docker runs: two selected cases
passed per run; six other cases were deliberately filtered. The unchanged cases
can pass outside the failing full-suite context; the underlying cause remains
unproven and the product contract is not claimed repaired.

| Quarantine  | Exact case                                                                                                                           | Owner                                                        | Observed failure and disabled coverage                                                                                                                                                           |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Q-STORY-001 | `catalog/components/store-application/StoreWorkspaces.stories.svelte` / `KeyboardSearchAndRepositoryOverview`                        | jj-store maintainers: workspace picker and catalog lifecycle | The full-suite run could not find `No matching workspaces.` after typing an unmatched query. Quarantine leaves the empty-state, keyboard selection and repository-overview sequence unqualified. |
| Q-STORY-002 | `catalog/components/store-appearance/AppearanceOverlay.stories.svelte` / `SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes` | jj-store maintainers: appearance runtime and overlay catalog | At `everforest-light`, the overlay retained `rgb(45, 53, 59)` while the app sample was `rgb(253, 246, 227)`. Quarantine leaves the open search/menu palette-transition sequence unqualified.     |

To diagnose or re-enable a case, remove only its `quarantined` story tag in a
disposable source change; retain all play assertions and diagnostic guards. Use
the configured remote Docker runner with locked registry dependencies and no
machine-local package archive overrides. Run each original target:

```sh
pnpm test:storybook catalog/components/store-application/StoreWorkspaces.stories.svelte -t KeyboardSearchAndRepositoryOverview
pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte -t SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes
```

The owner must identify and repair the underlying cause at its owning source
boundary, retain the exact original assertions, and pass three fresh-process
exact runs, both complete story families, and three complete cold four-shard
interaction/accessibility runs with the tag removed. Record the actual counts
and receipts in new evidence before removing the register entry. Any newly
exposed failure remains blocking. Wider product, provider, platform,
real-storage and visual-baseline qualification remain separate.

### Reproducing the audit profile

The audit uses four CPUs and 24 GiB, registry-locked dependencies and the saved
remote transport/context. The following invocation leaves the saved local
configuration and package archives untouched. Replace `test --no-fail-fast` with
a targeted lane and its arguments when diagnosing an individual case.

```sh
node --input-type=module - test --no-fail-fast <<'JS'
import { readFile } from "node:fs/promises";
import { validationConfig } from "./scripts/validation/model.mjs";
import { remoteRun } from "./scripts/validation/runs.mjs";
const local = JSON.parse(await readFile(".local/validation/config.json", "utf8"));
const [lane, ...args] = process.argv.slice(2);
await remoteRun({
  ...validationConfig(local),
  runtime: "docker",
  cpus: 4,
  memoryGiB: 24,
  archives: undefined,
}, lane, args);
JS
```

### Native Git test prerequisites

The full remote validation image provides actual Git 2.55.0 for the existing
`jj-git-sha1-0.45.1-v1` transport profile. It builds the released source archive
from kernel.org after checking its pinned SHA-256 and verifies the installed
version. HTTPS support and an SSH daemon with its privilege-separation directory
support the existing generated-credential, loopback-only transport fixtures. The
isolated image account is unlocked for those generated public-key logins; the
fixture disables password authentication and supplies its own host keys. Tests
still own and reap their own temporary servers; no default SSH service is
started by the runner. A system Git with a different version is not substituted,
and the native capability guard and credential assertions remain unchanged. This
tooling correction does not promote Linux deployment or release gates.

### Headless desktop account

Desktop terminal permission acceptance requires an ordinary account. Root can
execute a regular file with any execute bit and cannot qualify the negative
owner-permission fixture. Build the headless desktop workspace through managed
Cargo, discover every test executable from its `compiler-artifact` JSON output,
and run the complete inventory with its standard harness under an isolated
unprivileged container account. Retain existing prerequisite ignores and also
run the workspace documentation tests. Record the source snapshot, account,
exact permission regression and complete binary results. A root-only fixture
failure is not quarantined or counted as passing desktop acceptance.

### Standalone metadata checks

Dependency-boundary checks require the full image's actual Cargo toolchain for
workspace metadata. Spec-first pairs the transferred source revision with its
transferred first parent unless the caller supplies an explicit comparison.
Explicit head/base arguments, including equal-sign forms, remain selected
without duplicate options. A source with no available comparison parent retains
the validator's ordinary context inference. These prerequisites do not weaken
source identity, specification, dependency or authorization gates.
