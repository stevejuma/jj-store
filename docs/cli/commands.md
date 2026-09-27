# Native commands

- `jj store attach`: create a physical workspace for an existing repository.
- `jj store status`: inspect authoritative operation heads.
- `jj store doctor`: check binding and current access without publishing.
- `jj store login` and `logout`: manage one OS credential account.
- `jj store capabilities`: inspect verified and unqualified capabilities.
- `jj store version`: inspect build information.
- `jj store help -k credentials`: credential and TLS guidance.
- `jj util completion`: upstream completions including the store command family.
- `jj util markdown-help`: raw authored command help in Markdown.

- `jj store serve-local --server-config PATH`: run the paired foreground server.
- `jj-store run --config PATH`: run the standalone server.
- `jj-store config`, `doctor`, and `operator`: configure, inspect and administer
  it.

Repository creation commands remain under implementation; the milestone ledger
is the authority for completion.
