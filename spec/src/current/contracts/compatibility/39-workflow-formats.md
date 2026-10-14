# Workflow compatibility

JST-WF-001 adds an optional WorkflowStore root and state version in manifest
format 6. It requires the existing WorkStore home root, retains frozen origin
metadata, and participates in native copy, selected history and recovery.
Manifests without workflows retain their existing serialization. Older binaries
cannot serve a format-6 home; upgrade the paired server and custom CLI together.
There is no automatic format downgrade or WorkflowStore origin reducer.

Workflow HTTP/native RPC documents and the operator journal use version 1.
Execution-host principal admission defaults empty and is activated by explicit
workflows configuration reload without restarting the server. Optional managed
host configuration adds fields to the existing version-1 settings API; the
standalone operator JSON remains byte-shape compatible. Upgrade paired binaries
once to expose this capability. Busy or uncertain work defers replacement. Host
journal generation participates in stable request identity. Unknown attempts
retain their source, owner and report identity and cannot be replayed or taken
over automatically. This does not guarantee exactly-once external effects.

The initial executor profile is named Unix commands plus existing admitted
AgentManager APIs. Current execution qualification is macOS ARM64 and the pinned
embedded RustFS runtime. Local `forgejo-runner exec` is an operator wrapper;
unchanged remote Forgejo Runner registration/polling, full Actions expression
semantics and ConnectRPC are not implemented by this profile. See JST-WF-004,
JST-WF-005 and the M28 ledger for live provider/runner and visual approval gaps.

Version-1 definitions add optional conditions. Empty conditions and absent
conversation fields are omitted so existing canonical records and source event
identities re-encode identically. Conversation sources have optional code pins
and exact typed ThreadStore/ReactionStore references; no code anchor is
invented. Condition results are trusted observations from explicitly admitted
hosts. Source message bodies and reaction content remain in their owning
repositories.

Managed runner environment settings version 2 coexist with standalone/managed
version-1 host attachment JSON. Native WorkflowStore objects, manifest format 6,
host journals version 1 and reporting receipts do not change. Unknown settings
fields or versions fail validation. UI edits project legacy environment fields
into version 2 only when saved; saved legacy mappings remain available in JSON
and no uncertain job is replayed during conversion.

The first-party `github_workflow` executor extends version-1 definitions with a
selected safe YAML path. Optional log `step` scope, bookmark `code_context` and
run-detail `event` fields are omitted when absent; legacy objects and event IDs
retain their bytes. `skipped` is a terminal run/step status. Upgrade the paired
server, CLI and browser before writing these variants; older readers cannot
interpret them. Manifest format 6, root/catalog versions and host journals stay
unchanged. Existing duplicate evaluations retain their first recorded context.

An optional `forgejo` capability extends both host configuration profiles with
operator-owned program/Docker paths, image, architecture, environment and secret
references. Settings use the existing save/reload boundary. Runner v13.2.0 is
qualified separately from remote registration/polling/ConnectRPC. Linux Docker
jobs run from native exports; no Git metadata or GitHub API/token is invented.
Supported YAML filters use push/workflow_dispatch, branches/branches-ignore and
paths/paths-ignore with ordered negation and bounded glob patterns. Glob `+`,
workflow concurrency, dispatch inputs, dynamic runs-on, services, job container
overrides and reusable workflow jobs fail explicitly in this profile. YAML
branch filters do not match unbranched code publication. Missing/ambiguous path
comparisons fail instead of matching. Checkout retains the exported tree.

For local execution, `github.sha` carries the full native commit identity.
Unbranched source uses `refs/jj-store/commits/<native-id>`; bookmark sources use
`refs/heads/<bookmark>`. These are execution-context strings, never published
Git objects or a clone endpoint. YAML trigger filtering always uses the original
semantic event and does not treat the native namespace as a branch. The private
execution copy inserts a standard pinned-source checkout when none exists.
Checkout repository/ref/path overrides are refused.

The private execution name contains the attempt/action identity to isolate
Forgejo's Docker resource names across hosts. `github.workflow` therefore uses
that internal name. Workflows depending on the original workflow-name context
are not qualified; full GitHub context compatibility remains outside this slice.

Run pages add bounded selected/latest `step_reports` independently of log
pagination. The console merges these immutable reports, displays actual YAML
steps and retains executor/job setup output under Run output. Both raw Actions
`::group::`/`::endgroup::` and normalized Forgejo `##[group]`/`##[endgroup]`
markers produce nested disclosures. Scoped log reads retain trailing output
after a step's terminal report, including across page boundaries.
