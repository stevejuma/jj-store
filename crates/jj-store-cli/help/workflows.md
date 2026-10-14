# Native workflows

Use a native WorkStore repository as the metadata home. Create its project first
with `jj project create`. Linked code repositories remain independent and need
their own access grants. Workflow writes select manifest format 6; use matching
server/CLI versions. Workflow homes retain the WorkStore origin-replication
freeze.

`jj -R HOME workflow list`, `show ID` and `runs --workflow ID` inspect
definitions and history. `put --request-id REQUEST FILE.yaml` creates a
disabled-by-default definition unless its YAML explicitly enables it. Updates
take `--id ID` and `--expected VERSION`. `enable`, `disable` and `remove`
require the current version and a stable request ID; removal retains historical
definitions and runs.

```yaml
name: Build
enabled: true
repository: SOURCE_REPOSITORY_ID
trigger:
  on: bookmark_changed
  bookmark: main
labels: [linux]
actions:
  - id: build
    executor:
      type: command
      name: build
    inputs: {}
    timeout_seconds: 300
```

Replace placeholders with full hexadecimal IDs. Native repository/request/run
IDs use 32 characters; operation and commit IDs use 128. Triggers are `manual`,
`code_published`, `bookmark_changed`, `message_posted` and `reaction_added`.
Actions run in order and stop on failure. The definition names an admitted
command or agent setup, never an executable.

`run ID --request-id REQUEST --expected VERSION --source-repo SOURCE --operation OPERATION --commit COMMIT`
creates a manually selected run. `cancel RUN` requests termination; a running
attempt becomes cancelled only after its owner confirms termination.
`retry RUN --expected ATTEMPT` creates a new attempt using the same definition
and source. Unknown attempts cannot be retried or reassigned.

Each mutation retains its exact request ID and body. `status REQUEST` recovers
the original durable receipt. `logs RUN --follow` follows the current attempt;
`--attempt ATTEMPT` selects a historical one. `runs --run RUN --at POSITION`
pins recorded history.
`report RUN --request-id REQUEST --attempt ATTEMPT --sequence NUMBER FILE.json`
appends a typed step, log, artifact or agent reference for the assigned owner.
It requires execution-host admission as well as home Write.

## Browser actions

The workflow card menu offers Edit workflow, Run workflow, Duplicate and Remove
workflow with icons. Run workflow opens a Settings-style source form. Search the
recorded operations, then select a commit or bookmark from that exact operation.
Choices load in bounded pages; use Load more for additional recorded values.
Search filters the loaded choices. Changing the operation clears the commit and
bookmark. A bookmark with one unambiguous target selects its exact commit;
conflicted or targetless bookmarks cannot be selected.

Overview cards summarize the trigger, first command or agent and remaining
steps, with stacked action icons. All cards, including Create workflow, have the
same fixed height. The workflow name, source repository and version date remain
in the footer. Create/Edit Form includes an optional Description field in the
trigger inspector; it uses the same stored description as YAML.

Source access is separate from metadata-home access. Lookup failures use retry
toasts and preserve selected values. After an unknown run outcome, source fields
stay read-only while Check original request reconciles the retained command.
Selecting source values does not publish a revision or execute the workflow.

## Optional event/action host

Open **Settings → Local service → Workflows** as a local administrator. Save
execution principal admission and an optional local host, then select **Reload
saved settings**. Admission defaults empty. Ordinary repository Write or agent
access does not grant host admission. The same admission is also available as
`workflow_hosts = ["PRINCIPAL_ID"]` in top-level server TOML.

Save is revision-conditional and does not start work. Reload's `workflows`
domain activates admission and an optional CLI-owned host without restarting
listeners, changing storage or resetting history. Enabled hosts poll
continuously. A poll, active attempt or retained uncertain request defers
replacement; reconcile the original request and reload again when idle.
Disabling waits for this boundary. The runtime status distinguishes stopped,
polling/running and busy state.

Local settings store `workflow_host.enabled` and `workflow_host.configuration`.
The managed runner uses version-2 environment settings, for example:

```json
{
  "version": 2,
  "host": "local-builder",
  "labels": ["linux"],
  "commands": {
    "build": { "program": "/usr/bin/make", "args": ["test"] }
  }
}
```

No principal, metadata home or source repository paths are required. The runner
uses installation-registered native workspace links, verifies repository
identity and the selected service's socket, then checks each job's recorded
repository and exact operation/commit. Only homes with a WorkflowStore root are
polled, with a separate recovery journal per home. Ready native attachments are
bounded to 32; unavailable or changed links never substitute another source.
Admission is still explicit: open Execution access and Admit local host, Save,
then Reload saved settings. Discovery and enabling a host do not grant
admission.

The normal form shows enablement, name, labels, command presets and agents.
Advanced starts collapsed. Leave `state_dir` and `jj_executable` empty for a
private installation host directory and the paired custom JJ CLI. Polling
normally uses 1000 ms. Empty `allowed_owners` uses this installation's native
and browser local-service identities. Agents use the existing local dispatch
endpoint unless `agent_endpoint` is explicitly supplied; existing agent setup
and source-target admission still applies. Command programs, environment and
secret references remain operator-owned and editable through highlighted JSON.

Version-1 configuration below remains supported by standalone and managed hosts.
Form edits project its execution fields into version 2 on explicit Save,
preserving custom recovery paths, JJ path, owners, commands and agents. An
existing matching home journal remains at its original path. Active or uncertain
work defers replacement; state is never reset. This server's connection supplies
identity; source access remains independently enforced. Reload applies later
configuration changes without replacing listeners. Updating the binary itself
requires one development backend restart.

Run `jj-store workflow-host --config /ABSOLUTE/host.json`; `--check` validates
without starting and `--once` observes one poll and completes selected work.
`jj-store run --config SERVER.toml --workflow-host /ABSOLUTE/host.json` runs the
same optional host alongside the server. Core publication does not depend on it.

The private operator JSON and state directory must be outside repository
checkouts:

```json
{
  "version": 1,
  "host": "linux-builder",
  "principal": "PRINCIPAL_ID",
  "home": "/ABSOLUTE/attached-home",
  "state_dir": "/ABSOLUTE/private-workflow-state",
  "jj_executable": "/ABSOLUTE/custom-jj",
  "allowed_owners": ["DEFINITION_OWNER_ID"],
  "labels": ["linux"],
  "sources": [
    {
      "repository": "SOURCE_REPOSITORY_ID",
      "workspace": "/ABSOLUTE/attached-source"
    }
  ],
  "commands": {
    "build": {
      "program": "/usr/bin/make",
      "args": ["test"],
      "environment": { "PATH": "/usr/bin:/bin" },
      "secrets": { "DEPLOY_TOKEN": "MY_HOST_DEPLOY_TOKEN" },
      "artifacts": ["result.txt"]
    }
  },
  "agents": [],
  "agent_endpoint": null,
  "poll_millis": 1000
}
```

Every required label must match the host. `allowed_owners` admits authors whose
definitions may execute. Secret mappings contain environment-variable names, not
values; values stay in host/child memory and are redacted across log chunks.
Configure minimal ordinary environment variables explicitly: children receive an
empty inherited environment. Command arguments support `{workspace}`,
`{repository}`, `{operation}` and `{commit}` substitutions. Children also
receive `JJ_WORKFLOW_RUN`, `JJ_WORKFLOW_ATTEMPT`, the three source IDs and JSON
`JJ_WORKFLOW_INPUTS`. Each attempt starts in a fresh pinned source directory;
its ordered command actions share that directory so later actions can use
earlier outputs. A source without a code anchor starts in one empty directory.

Agents use `{ "type": "agent", "agent": "SETUP_ID", "prompt": "..." }`. Admit
that setup under `agents` and configure `agent_endpoint` with `url`, `local` and
optional `token_environment`. Local endpoints require literal-loopback HTTP;
remote endpoints require HTTPS and an environment credential reference. Source
`target_id` selects an already admitted execution attachment where required.
Existing AgentManager authorization, sandbox policy, receipts and transcripts
remain authoritative. No new inference provider is introduced.

The host journals each command before reporting through the custom CLI. Source
observation starts at the current publication on first startup; it does not
backfill old changes automatically. Multiple hosts share one native home and
claim each event/run once. A lost response retains the original request. Host
restart marks an interrupted claimed attempt unknown and never relaunches its
process or prompt. The journal retains bounded diagnostics for ambiguous source
events. Preserve it for recovery; deleting state is not an execution retry.

## Source and compatibility limits

`jj -R SOURCE workflow source --repo-id SOURCE_ID --operation OPERATION --commit COMMIT /ABSOLUTE/new-directory`
exports a visible pinned tree without snapshotting or publishing source
operations. Conflicted files, symlinks, submodules and reserved paths are
refused. Limits are 100,000 files, 32 MiB per file and 512 MiB total. Logs are
bounded to 10 MiB per action; inline artifacts to 128 KiB each. Command
process-group supervision is currently qualified on macOS ARM64 only. Commands
are operator-admitted local execution, not a new untrusted-code sandbox.

An operator command may wrap
`forgejo-runner exec --workflows .forgejo/workflows` using the pinned directory
and report data through the CLI. The runner parses Actions YAML. Its local
executor skips checkout unless configured otherwise; keep execution on the
exported revision. This is an adapter boundary, not registration/polling
compatibility with unchanged remote Forgejo Runners. See
[Forgejo Actions](https://forgejo.org/docs/latest/user/actions/overview/) and
the runner's
[local exec command](https://code.forgejo.org/forgejo/runner/src/branch/main/internal/app/cmd/exec.go).
Live Forgejo-wrapper and provider execution remain separate qualification gates.
Cron, webhooks, DAG scheduling, automatic retries/failover and ConnectRPC remain
deferred. Workflow YAML here describes the small event/action host; it is not
GitHub Actions YAML.

## Trigger conditions

Bookmark names accept literal names plus `*` and `?`. Code triggers may restrict
the pinned revision with a native JJ `revset` and commit-author emails. Thread
message and reaction triggers use authenticated principal IDs, optional exact
`thread`/`message` IDs, and case-sensitive text comparisons for new messages.
Reaction values use a Unicode emoji, `+` (Like), `-` (Dislike), or `custom:ID`.

```yaml
trigger:
  on: message_posted
conditions:
  authors: [PRINCIPAL_ID]
  thread: THREAD_ID
  text:
    operator: contains
    value: deploy
```

`excluded_authors` and `excluded_emoji` provide negative matches. Text operators
are contains, not_contains, starts_with, ends_with, equals, not_equals, is_empty
and is_not_empty. All supplied conditions must match. Message edits/tombstones
and reaction removals do not retrigger. `workflow events --filters FILE.json`
evaluates an array of conditions against pinned source publications. Invalid
revsets pause observation without advancing the cursor. Definition selection is
pinned before polling; a concurrent edit applies to subsequent observations.

Conversation runs retain source references and publication position, rather than
copied message bodies. Without a code revision, command actions start in an
empty private workspace. `JJ_WORKFLOW_SOURCE` contains their typed source JSON;
existing code runs still export the exact pinned operation/commit. Agent prompts
include the immutable trigger references and use the admitted source attachment.
Thread/Reaction origin replication remains deferred with their existing frozen
home origin profile.

## GitHub YAML executor

Configure an optional `forgejo` runner capability in Workflows settings, with
absolute Forgejo Runner 13.2.0 and Docker executable paths, a job image and
admitted labels (for example `ubuntu-latest`). On macOS, set the Docker endpoint
from `docker context inspect --format '{{.Endpoints.docker.Host}}'`. The native
macOS runner can be built from the pinned upstream source using Go. Saving does
not activate the host: use the existing saved-settings reload and explicit host
admission.

Choose **GitHub workflow** for an action and select a `.github/workflows/*.yml`
or `.yaml` path. App criteria select candidates; the YAML's supported event,
branch and path filters must also match. The workflow body and source are read
from the same recorded revision, including edits introduced by that revision.
Reruns retain original pins. A YAML non-match or absent file is skipped.

The runner executes Docker jobs without a Forgejo server. `push` maps to
bookmark changes or unbranched code publication; manual runs require
`workflow_dispatch`. Include the workflow file in `paths` if workflow-only edits
should trigger it. Initial limits and unsupported fields are listed in the
canonical workflow compatibility contract. GitHub-hosted tokens/APIs and
artifact/cache services are not provided. Secret settings contain operator
environment names only.
