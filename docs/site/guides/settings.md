# Settings and maintenance

Settings separates device presentation preferences, installation operation and
repository-specific management. Read the selected context before changing a
setting: a device appearance preference does not change repository metadata, and
a repository policy change is not a cosmetic preference.

The visible Settings section reacts to its own events. Storage statistics,
search health, folder registrations and configuration do not repeatedly fetch
while unchanged. Feed recovery reconciles missed changes; unavailable feeds use
bounded fallback only while visible.

## Saved and effective settings

Where settings expose both saved and effective values, saved configuration is
what a future activation will use; effective state describes the current
worker/service. Follow Apply or restart indicators. A successful configuration
save alone does not prove a worker restarted or a long-running operation
completed.

Server inventory includes values that are intentionally read-only. Storage
location, access transitions, launch configuration and backup execution are not
made editable merely because their current values are listed. Keep secret
references separate from secret values.

## Workspaces and capture

Workspace management links verified native workspaces or explicitly admitted JJ
folders to the device. It does not migrate or import a repository implicitly.
Capture settings expose the configured source profile, roots, worker state and
retained original import/control request identities. See [capture](capture.md)
for the difference between pause/resume and startup configuration.

Eligible repository settings expose **Repository storage** for an independent
local/native copy. Preflight and retained request status precede opening the new
destination; see [repository conversion](conversion.md). This does not change
the meaning of device folder registration or workspace links.

## Search and sync

Search can be disabled independently of repository authority. Preparing an index
is an explicit bounded action; inspect its result rather than assuming that a
spinner disappearing means a complete index. Sync exposes configured worker
state, finite waits, pause/resume and explicit reconciliation. A remote outage
does not require moving ordinary reads to the upstream.

Repository Advanced settings can prepare the selected graph index and show its
retained task status. Use **Cancel preparation** to stop that task and wait for
the confirmed outcome. Navigation may stop observing it; a canceled HTTP request
alone does not establish that the server worker stopped.

## Integrity and checkpoints

Storage integrity checks operate on a prepared exact target with finite budgets.
An incomplete report means verification did not finish within those limits; it
must not be interpreted as healthy. Retain the target identity and report when
investigating. A small bounded repository can have a complete report while a
larger installation reaches the time budget.

Non-destructive checkpoints retain recovery semantics and durable receipts. They
are not an export archive or a substitute for qualified backup/restore.
Destructive garbage collection, automatic promotion and multi-writer serving
remain disabled. Do not delete candidates, journals or storage files manually to
resolve a reported maintenance condition.

## Local Stacks workspace settings

For a registered ordinary JJ repository, open **Stacks → Workspace actions →
Settings**. The Details table starts with the copyable Workspace ID, followed by
the active workspace name and target branch. The branch picker searches
available unambiguous heads. Save reviews the exact change; Cancel leaves the
saved configuration unchanged.

Deleting a workspace requires confirmation. It removes the saved workspace
configuration, preserving the checkout, branches, commits and unfinished native
JJ work. Settings are stored locally in `.jj/jj-store/stacks.toml` and do not
enter repository history. See
[local stack workspaces](../technical/local.md#local-stack-workspaces) for
retention and unknown-command outcomes.
