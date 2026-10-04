Create a new native repository and attach its first physical workspace.

```sh
jj store init ./work --endpoint https://store.example
```

The destination must be empty for the first attempt. `--workspace`, `--cache`,
`--state-dir`, `--ca-certificate` and `--insecure-loopback` follow attachment
conventions. Use `jj store attach` for a repository that already exists.

Repository creation requires a separate operator credential, provisioned with
`jj-store operator init` or injected through `JJ_STORE_OPERATOR_TOKEN`. The
initial repository admin credential comes from `JJ_STORE_TOKEN` for automation
or is generated and saved to the OS credential facility before creation. Secrets
are never printed or stored in the checkout or journal.

Retry the same command after an interruption. Its durable local record retains
the repository ID, epoch and credential identity before contacting the server;
attachment resumes with its original operation. Changed options or credentials
fail explicitly. A retry of an acknowledged repository requires current access
but does not need the creation operator. Keep the state directory for recovery.
