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
