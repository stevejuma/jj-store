# Environment variables

Environment variables belong to the process that reads them. They are not
portable repository metadata, and changing a shell's environment does not alter
an already-running service.

| Variable                         | Consumer and purpose                                                                            |
| -------------------------------- | ----------------------------------------------------------------------------------------------- |
| `JJ_STORE_TOKEN`                 | Custom CLI bearer credential override; takes precedence over the selected OS credential account |
| `JJ_STORE_OPERATOR_TOKEN`        | Separate repository-creation operator credential                                                |
| `JJ_STORE_PAGER`                 | Custom text pager command; empty or `cat` disables it                                           |
| `JJ_STORE_SEARCH_TOKEN_…`        | Names referenced by authenticated search entries; values are per-repository read credentials    |
| `JJ_STORE_REPLICATION_TOKEN_…`   | Names referenced by replication entries; values authorize the configured upstream repository    |
| `JJ_STORE_GIT_CREDENTIAL_HELPER` | Approved Git helper choice: `none`, `osxkeychain`, `manager`, `libsecret`                       |
| `GIT_ASKPASS`, `GIT_SSH`         | Explicit absolute approved Git helper executable paths                                          |
| `GIT_SSL_CAINFO`                 | Explicit absolute CA file for applicable Git transport                                          |
| `CODEX_HOME`, `HOME`             | Default Codex capture root resolution when roots are omitted                                    |
| `CURSOR_DATA_DIR`, `HOME`        | Default Cursor CLI transcript root; Desktop uses `HOME/.cursor/projects`                        |

## Secret handling

Inject credentials from the appropriate private secret facility into the process
that needs them. Put the variable's name in configuration, not its value. Do not
place secrets in command arguments, repository files, URLs, copied logs or
browser storage. Browser tokens remain in memory. Logging out does not remove an
inherited environment override from your shell.

The external-storage schema additionally accepts environment references for
access key, secret key and optional session token. Those belong to server-side
storage, not native repository clients. External-provider qualification remains
separate from managed local storage.

## Development-only variables

| Variable                                                                 | Purpose                                                                                           |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
| `JJ_STORE_DEV_ROOT`                                                      | Explicit absolute existing development fixture root; use consistently for every lifecycle command |
| `JJ_STORE_DEV_SERVER`                                                    | Existing backend executable used by UI development mode                                           |
| `JJ_STORE_DEV_RUSTFS_RUNTIME`                                            | Explicit embedded-storage executable                                                              |
| `JJ_STORE_DEV_PORT`                                                      | UI loopback port, normally 7460                                                                   |
| `JJ_STORE_DEV_BACKEND_PORT`                                              | UI-mode backend port, normally UI port plus two                                                   |
| `JJ_STORE_DEV_BROWSER_ANNOTATIONS`                                       | Development annotation behavior; not production acceptance                                        |
| `JJ_STORE_CAPTURE_PREVIEW_ROOT`                                          | Isolated capture-preview fixture location                                                         |
| `JJ_STORE_CAPTURE_PREVIEW_PORT`, `JJ_STORE_CAPTURE_PREVIEW_BACKEND_PORT` | Capture-preview listeners                                                                         |
| `JJ_STORE_CAPTURE_CODEX`                                                 | Explicit Codex executable used by the preview harness                                             |
| `JJ_STORE_DESKTOP_DATA_DIR`                                              | Explicit development desktop data location                                                        |

Use [development commands](../contributing/development.md) rather than changing
service defaults to accommodate a temporary fixture. Test contract-capture and
fault-injection variables are internal harness interfaces and are not supported
application configuration.

## Interactive execution

`JJ_STORE_ACP_EXECUTABLE` enables the LocalTrusted agent supervisor using an
absolute control-v2 harness binary. `JJ_STORE_AGENT_JJ_EXECUTABLE` selects the
paired custom JJ; otherwise the service uses its sibling `jj` executable.
`JJ_STORE_AGENT_PRIVACY_CACHE` optionally selects the privacy scanner cache
outside checkouts. These are trusted launch configuration; browser requests
cannot override them. Adapters remain separately installed and authenticated on
the service PATH. See [agent sessions](../guides/agent-execution.md).
