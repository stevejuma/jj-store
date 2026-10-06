# Local Stacks

Run `jj-store stack -C PATH status` in an ordinary Git-backed JJ repository. The
CLI shares the application's native engine and `.jj/jj-store/stacks.toml`. It
needs no server or registration. Native JJ owns commits, conflicts, the target
megamerge and its unfinished working-copy child.

## Start and inspect

```text
jj-store stack init --name development --target main
jj-store stack list
jj-store stack apply feature/api
jj-store stack show feature/api
jj-store stack status --format json
jj-store stack workspace create experiment
jj-store stack workspace switch development
```

Normal commands snapshot current files with JJ ignore and tracking rules.
Inspection with `--no-snapshot` reads the last snapshot. A mutation with
`--dry-run` validates and describes its action against the last snapshot,
without snapshotting, creating settings, reconciling heads or checking out
files. Resolve concurrent operation heads with native JJ first. Use
`--expect-operation` and `--expect-revision` together to reject state changes
since inspection.

Apply saves exactly the selected bookmark tip. Unapply removes the column's
exclusive explicitly applied members together, retaining shared dependencies.
`unapply BOOKMARK --branch-only` removes one saved member. Integrated bookmarks
remain available through `list --all`. Before workspace setup use
`list --target BOOKMARK`. Show returns bounded per-card history; continue using
the returned operation/revision, `--card BOOKMARK` and `--after COMMIT`.

## Edit native changes

```text
jj-store stack create feature/new --parent feature/api
jj-store stack commit --branch feature/api -m "Update API" -- src/api.rs
jj-store stack commit --branch feature/api -m "Checkpoint" --all
jj-store stack commit --branch feature/api -m "Empty change" --empty
jj-store stack reword 'feature/api' -m "Updated description"
jj-store stack transfer --to feature/api -- README.md
jj-store stack diff --from @
jj-store stack hunks README.md --format json
```

Revisions use native JJ selectors and must resolve one visible commit. Paths are
relative to `-C`; selected paths identify files, not inferred directory
contents. `--all` selects all changed paths from the exact source. `--from REV`
changes the source from WIP; split defaults to its selected commit. Net branch
diffs are inspection only. Explicit messages use `-m` or `--message-file FILE`.

For partial changes, pass `--hunk ID` repeatedly with exactly one path and the
`--expect-operation OP --expect-revision REV` returned by `hunks`. Tokens are
native byte-bound selections; stale input fails without regenerating tokens.
Mixed-source input uses `--selection-file FILE` with this transient JSON shape:

```json
{
  "version": 1,
  "operation": "full native operation ID from status",
  "revision": 1,
  "selections": [
    { "source": "full commit ID", "path": "README.md", "hunks": ["native ID"] }
  ]
}
```

Omit `hunks` to select the whole changed file. Selection files retain exact
source IDs and canonical repository paths; they do not persist staging state.
Use native `jj log` and `jj op log` for general history.

## Conflicts and recovery

```text
jj-store stack conflicts
jj-store stack conflicts README.md --format json
jj-store stack resolve README.md --file reviewed-resolution.txt
jj-store stack discard --yes -- README.md
jj-store stack reconcile --dry-run
jj-store stack reconcile --yes
jj-store stack undo
```

Resolve submits a complete supported UTF-8 text resolution. `--file -` reads
stdin. Structural, binary, mode and oversized conflicts report their existing
unsupported reason. Undo/Redo uses repository-wide native operations, including
ordinary CLI changes. TOML membership is separate and never automatically
reapplied after native history changes.

Workspace/bookmark deletion, abandonment, discard and reconciliation require
`--yes`; apply/unapply execute immediately. Reconciliation is an explicit graph
repair, with guarded cleanup of proven obsolete app-owned merges. Read its dry
run first. Deleting workspace configuration retains native unfinished work.

Text supports existing paging and color controls; JSON is never paged.
Successful results include operation/configuration identities and warnings. Exit
codes: 0 success, 1 execution failure, 2 invalid arguments, 3 an operation
applied but checkout/settings need attention. For exit 3 inspect the reported
operation and do not repeat the command. No command automatically retries a
failed mutation. Remote/PR commands and terminal pickers are outside this local
command family.
