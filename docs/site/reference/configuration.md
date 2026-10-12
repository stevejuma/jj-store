# Configuration

The server consumes strict **format 2 TOML**. `jj-store init --local` generates
an installation-specific configuration and private identity files. Prefer that
output to manually constructing installation state. `config example` instead
shows the retained authenticated/external-provider schema; it is not the normal
LocalTrusted bootstrap path.

```sh
jj-store config check --config /absolute/private/server.toml
jj-store run --config /absolute/private/server.toml
```

`config check` parses and validates without contacting storage. Startup checks
runtime ownership, storage and credentials separately. Unknown keys and older
formats are rejected. Relative paths resolve against the configuration file, not
the shell's current directory. Secret values are never accepted in TOML; use the
supported environment or OS-credential references.

## Configuration locations in the app

Profile-menu **Settings → Local service** owns installation resource limits,
Protection, authority storage and backup inventory. Repository settings opens
from the repository page's gear: **Search** edits that repository's search
configuration, and **Sync → Configuration** edits its upstream and worker
limits. These locations retain the same installation-owned configuration file,
save permissions and explicit activation requirements. Moving a control does not
move its persisted data or introduce a new configuration format.

**Add server** above Help saves a device-local connection outside repositories.
Selecting a server scopes its repositories, projects, agents and directory to
that installation. Appearance remains device-owned. Public selection and
destinations resume after restart; sign in again because credentials are
memory-only. Unknown connections require explicit recovery.

The gateway's activity WebSocket uses the registered remote origin and retained
installation identity. It never forwards the local browser's Origin or local
authority. Keep the remote `public_url` consistent with its registered address;
normal Host/Origin, token and repository admission still apply. Replacing the
installation at an address invalidates the old connection session.

Authenticated servers can opt into operator-owned `[remote_agents]` definitions,
paired executable paths and per-agent principal allowlists. Repository Write
grants remain independent. Remote callers select existing native WorkStore homes
and native attachments; runtime configuration stays operator-owned. See
[server configuration](../cli/jj-store.md#authenticated-remote-agents) for the
configuration shape. M27 retains separate installed-runtime, storage and
platform gates.

## Top-level fields

| Key                                  | Meaning                                                                 |
| ------------------------------------ | ----------------------------------------------------------------------- |
| `format`                             | Required format number, currently `2`                                   |
| `bind`                               | Listener socket address; LocalTrusted requires literal loopback         |
| `public_url`                         | Advertised browser/API origin; must match the admitted listener context |
| `insecure_loopback`                  | Explicitly permit plain HTTP for literal loopback                       |
| `state_dir`                          | Private installation state outside checkouts                            |
| `operator_policy`                    | Private repository-creation policy path                                 |
| `web_root`                           | Optional built application directory, including bundled `help` assets   |
| `development_browser_annotations`    | Explicit local-only annotation policy; false for normal strict serving  |
| `tls.certificate`, `tls.private_key` | TLS files for supported configured listeners                            |

## Access and storage

`[access]` uses `mode = "local-trusted"` or `"authenticated"`. LocalTrusted
names installation/catalog files, `rpc_socket` and `permission`. Authenticated
access names installation/catalog files and uses repository-scoped credentials
at request time. Changing a mode is not a supported implicit state migration.

`[storage]` uses `mode = "embedded-rustfs"`, `"filesystem"`, or the retained
`"external-s3"` schema. Managed RustFS names its runtime executable, persistent
volume, installation, credentials and ownership paths, loopback address, bucket
and region. The initializer creates the private state required by those paths.
Filesystem mode requires LocalTrusted Admin and rejects native search,
replication and backup entries. External S3 release qualification remains open.

Managed agent names, avatars and original setup associations remain private
installation state. Repository views may enrich recorded captures from their
exact admitted local associations. These read-only projections require no TOML
key or repository migration and remain distinct from portable AgentStore data.

The following complete filesystem configuration is parser-tested. Replace
`/DOCS_STATE` and `/DOCS_WEB` with actual initialized state and built asset
paths; the example does not create installation identity files.

<!-- config-example:filesystem -->

```toml
format = 2
bind = "127.0.0.1:7443"
public_url = "http://127.0.0.1:7443/"
insecure_loopback = true
state_dir = "/DOCS_STATE"
operator_policy = "/DOCS_STATE/operators.json"
web_root = "/DOCS_WEB"

[access]
mode = "local-trusted"
installation = "/DOCS_STATE/installation.json"
catalog = "/DOCS_STATE/repositories.json"
rpc_socket = "/DOCS_STATE/native.sock"
permission = "admin"

[storage]
mode = "filesystem"
```

## Request limits and protection

`[limits]` bounds concurrent requests, request seconds, native blob bytes and
inventory nodes. Defaults are [generated from Rust](defaults.md). These are
admission limits, not promises that every operation can finish on every dataset.
`[protection]` contains protected bookmark names and commit IDs. Semantic policy
is shared by RPC and application mutations; changing transport does not bypass
protection.

## Search

Each `[[search]]` entry names a unique repository, optional `enabled`, source
configuration and worker budgets. At most eight entries are admitted. Defaults
are 1,000 ms polling, 60-second timeout, 536,870,912 index bytes and a remote
snapshot every 32 committed source positions. A snapshot interval of zero
disables those optional snapshots.

Authenticated search requires a unique `token_environment` whose name begins
`JJ_STORE_SEARCH_TOKEN_`. LocalTrusted search derives its admitted principal and
must omit that credential field. Keep this difference when adapting an example
between access modes.

`source.scopes` maps labels to bookmark/workspace selections. Source settings
include `index_source`, include/exclude prefixes, maximum text bytes, files,
work and output bytes. Changing source identity rebuilds derived state. Index
paths live under installation search/adapter directories, not in checkouts.
Search faults degrade indexing independently of authoritative serving.

## Replication

Each `[[replication]]` entry binds one local repository to one upstream, a
`JJ_STORE_REPLICATION_TOKEN_...` environment reference and optional CA path.
Entries are unique by repository and limited to eight. The token needs the
appropriate upstream repository access. HTTPS is required except explicitly
admitted literal loopback HTTP.

`[replication.limits]` on the current entry bounds origins, closure entries,
objects, bytes, object size, polling, retries, circuit behavior and control
waits. See the [generated defaults](defaults.md) and
[sync guide](../guides/sync.md). Workers resolve credentials into memory; status
never returns them.

## Agent capture

`[agent_capture]` defaults to disabled. Resource limits cover readers, rescan
seconds, sources and pending bytes. Each harness entry names an exact profile
and product version. Transcript profiles accept optional approved roots;
OpenCode read-only API capture instead requires an explicit `endpoint` and no
roots. Its HTTP(S) URL must use a literal loopback address, root path, and no
credentials, query or fragment. Supported configured profiles are:

| Profile                        | Product version      | Source                                                                 |
| ------------------------------ | -------------------- | ---------------------------------------------------------------------- |
| `codex-cli-jsonl-v1`           | `0.156.1`            | Codex session JSONL                                                    |
| `codex-desktop-jsonl-v1`       | `0.159.2`            | Separate Desktop profile; installed-product qualification remains open |
| `cursor-desktop-transcript-v1` | `3.22.7`             | Cursor Desktop transcripts and explicit binding hooks                  |
| `cursor-cli-transcript-v1`     | `2026.09.26-dd393fe` | Cursor CLI transcripts and explicit binding hooks                      |
| `opencode-read-only-api-v1`    | `1.18.30`            | Explicit local read-only API                                           |

See [capture](../guides/capture.md) for setup and resolved root conventions.
Enabled automatic capture requires writable LocalTrusted native serving;
filesystem-only mode is not an alternative automatic native worker. The retained
OpenCode export profile is not enabled by the configured automatic reader UI.

Use **Reload saved settings** to activate capture configuration. The existing
worker drains before replacement; generation, pause policy, spools and original
import receipts remain retained. Runtime Resume does not override a disabled
saved/effective configuration.

## Saved settings and unsupported operations

UI settings may expose saved and effective values separately. Follow the field's
Apply/reload indication. Read-only inventory does not enable storage relocation,
access-mode transitions, backup execution or launch reconfiguration. Retained
`[[backup]]` schema is not a qualified backup/restore workflow. Use the
[capability reference](capabilities.md) before configuring a deployment profile.

The private retained Stacks reword executor now joins journal, publication and
unchanged-tree pointer proofs. Witness-bearing request catalogs use version 3,
retain exact recovery inputs and reserve bounded receipt/checkout growth.
Earlier catalogs remain readable and are not upgraded by status reads. This is
an internal installed-policy boundary; no new configuration option, command
route, general checkout or writable UI capability is exposed.

Retained Prepared first apply resolves the existing authorized device binding
before restoring exact source dependencies. Source-backed current policy runs
after restoration and still rejects changed combination/code selection or
physical edits before publication. Revoked bindings cannot restore source state;
historical terminal retry does not repair it. No setup or configuration is
inferred from recovery input.

Current immutable policy is rechecked for unpublished original Prepared/Unknown
reword requests. Already-selected original publication uses acknowledgement-only
recovery after policy changes; it cannot reannounce a vanished candidate. These
private checks add no configuration or production mutation capability.

A known Published receipt cannot reauthorize candidate publication before its
checkout witness is saved. Lost selection retains the original receipt pending
and requires explicit recovery of the original selection.

Agent runtime selection is installation-owned private journal state, outside
repositories and TOML configuration. Settings explicitly installs fixed Codex
tooling into a fresh private directory. Failed verification keeps the previous
selection; interrupted unselected staging is never replayed. Discovery/launch
resolve that bin directory first, followed by device PATH and conventional tool
directories. There is no npm-cache search or global binary replacement.

The local macOS JJ Agent can use the `chatgpt-plan` provider with an opaque
`options.account` registration reference. Application defaults and per-agent
selections use the existing conditional settings writes. Accounts are connected
in Settings → Agents → Defaults; tokens are kept in Keychain. This provider has
a fixed endpoint and does not accept API-key or endpoint overrides. Model and
effort defaults apply only to the same provider/account. Configure conversation
metadata independently, and explicitly restart after saved launch configuration
changes.

Object-backed servers advertise installation profiles and custom emoji without a
new TOML section. The configured object store and installation identity
determine the directory namespace. Startup verifies reachable selected
records/media closure. A temporary store outage preserves server startup while
personal settings remain unavailable; corrupt selected records fail closed.
Reads do not create default profile objects. `operator_policy` continues to name
a private local policy file and controls create-repository or manage-identities
capabilities. Profile details and token identity bindings live in the object
store. Filesystem-only servers advertise this capability as unavailable.

Authenticated native deployments also support `manage-agents` for the **Agents →
Manage server access** editor. It saves only existing `remote_agents`
configuration with a full-file revision check and shows saved and effective
values separately. Tools and provider credentials remain on the server; Save
does not activate changes. **Reload saved settings** activates definitions and
allowlists without restarting the server or installing tools. Identity
enrollment and repository Write grants require their separate permissions.

## Reload saved configuration

Use **Reload saved settings** in the top-right **Application menu** (the
vertical kebab). It applies only supported saved domains on the selected server
and preserves unsaved drafts. Local administrators can reload service limits,
protection, sync, search, capture and available agent/Compute settings. On
authenticated servers, an enrolled `manage-configuration` operator may reload
service, protection, sync, search and server agents; `manage-agents` permits
server agents only. Neither permission grants repository access or execution
authority.

The dialog reports Applied, Current, Deferred or Failed for each domain. Active
agent launch changes and Compute relay changes wait for their next worker or
helper start; use the existing worker controls when appropriate. Reload does not
restart the server, disconnect listeners, install tools or replay prompts.
Protection and sharing changes fence in-flight publication and apply to retained
authority owners. If acknowledgement is lost, **Check original outcome**
observes the same request. Do not submit a new intent while that outcome is
unknown.

Listener/TLS, identity/access, authority storage, executables and web bundle
paths remain startup-managed. Backup execution and storage migration are
separate workflows. The existing `restart_required` API field remains compatible
and indicates a saved/loaded mismatch for reloadable domains.
