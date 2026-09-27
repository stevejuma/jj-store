# CLI and server commands

First custom commands: init, attach, status, capabilities, doctor, serve-local,
and credential login/logout. JSON is never paged. Custom standalone no-command
invocation prints help successfully; ordinary upstream jj defaults stay intact.
Match jj-nostr presentation semantics without inheriting its storage or daemon
installation policy. Secrets stay out of argv, output, and checked-in config.

## Requirements

| ID          | Requirement                                                                                                                                                                                |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-CLI-001 | Custom commands MUST use jj store under upstream CliRunner, and the server MUST use explicit jj-store run with configuration, diagnostics, and token administration.                       |
| JST-CLI-002 | Custom command families MUST provide authored Markdown long help, short help, keyword help, hints, JSON results, TTY paging, no-pager, color controls, completions, and build information. |
| JST-CLI-003 | Ordinary JJ commands MUST preserve upstream behavior; a tested capability matrix MUST report unsupported backend commands honestly.                                                        |
| JST-CLI-004 | Development MUST use explicit build paths and MUST NOT replace installed jj or start unrelated services.                                                                                   |

## M0 extension qualification

The Apache-2.0 upstream example compiles at the pinned release and is exercised
through isolated init/describe/log/op-log/evolog commands. Its Git backend is
reference-only; native commands and presentation coverage remain M3.

The native library lifecycle fixture covers the stores used by the future custom
CLI, including independent local caches, repository attachment, transactions and
reopening. It is not executable command-presentation or remote CLI evidence.

The CLI integration library binds remote RPC authority and immutable object I/O
to the existing native factories. Loopback network lifecycle tests qualify this
binding separately from the future executable command presentation matrix.

## Executable native workspace client

The development `target/debug/jj` registers lazy remote factories with upstream
CliRunner. Ordinary JJ command parsing, transactions and workspace updates
remain upstream-owned. Each CLI invocation owns its factory cache and Tokio
runtime; there is no global mutable repository or principal. Native workspace
bindings contain the endpoint, repository, profile and transport limits, never
credentials.

`store attach` creates a separate physical workspace and local metadata
directory. Its destination must be empty and its logical workspace name must be
unique. `store status` and `store doctor` inspect remote heads without
snapshotting local files or reconciling operations. `store capabilities`
distinguishes verified native commands from unsupported or still-unqualified
ones. Executable creation, server administration and the remaining presentation
matrix stay gated by M3.

`store login` validates a token before storing it in the platform credential
facility; stdin or a hidden terminal prompt supplies the secret. `store logout`
deletes only the selected endpoint/repository credential. `JJ_STORE_TOKEN`
overrides stored credentials for automation and is never persisted. Custom text
results and authored help follow jj-nostr paging, Markdown and color
conventions; JSON results are plain, newline-terminated and never paged.

A hosted-RPC acceptance fixture creates a repository through the operator API,
then attaches and describes history with the actual custom jj executable. It
reopens the host with no in-memory repository directory. This qualifies dynamic
routing; init/serve-local and standalone server command presentation remain
separate gates.

## Standalone process

`jj-store run --config PATH` serves in the foreground and reports its bound
listener only after validating configuration, TLS and the operator policy.
`jj store serve-local --server-config PATH` uses the same composition and
requires a literal loopback bind address. Both stop on interrupt/termination and
bound graceful shutdown. Neither installs a daemon nor launches RustFS.

`jj-store config example` prints the configuration template, while
`config check` validates it without contacting storage.
`doctor --config PATH --repo-id ID` checks the configured repository through the
in-process authenticated service. It does not publish operations.
`operator init` persists a verifier for an environment-injected or OS-held
operator credential; `operator list/revoke` administer that trusted local policy
file. Token values are never command-line arguments or output. Repository token
commands remain a separate command slice.

On Unix, serve-local is parsed by CliRunner and then replaces the process with
the paired jj-store binary from the same executable directory. This resets the
upstream CLI's immediate-exit signal handler so the server owns graceful
shutdown. Other platforms fail explicitly and direct users to jj-store run;
platform handoff qualification remains a release gate.

Attachment records a bounded, secret-free local journal outside the checkout.
Before publishing it durably retains the exact operation and parent IDs; retries
use that identity through the native receipt path. Connection, credential ID,
workspace and path changes on retry fail explicitly. Cache initialization stages
complete bindings before rename. Physical metadata appears only after
publication is acknowledged; resuming never snapshots or overwrites source
files.
