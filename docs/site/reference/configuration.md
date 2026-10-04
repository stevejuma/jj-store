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
and product version, with optional approved roots. The current automatic Codex
profile is documented in [capture](../guides/capture.md). Enabled automatic
capture requires writable LocalTrusted native serving; filesystem-only mode is
not an alternative automatic native worker.

Capture configuration changes require backend restart unless the UI explicitly
provides an applicable activation action. Runtime Resume does not override a
disabled saved/effective configuration.

## Saved settings and unsupported operations

UI settings may expose saved and effective values separately. Follow the field's
Apply/restart indication. Read-only inventory does not enable storage
relocation, access-mode transitions, backup execution or launch reconfiguration.
Retained `[[backup]]` schema is not a qualified backup/restore workflow. Use the
[capability reference](capabilities.md) before configuring a deployment profile.
