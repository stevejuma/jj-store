# jj-store-local-agent

Application-owned launcher around the vendored Buzz ACP model/tool loop. It adds
managed provider/model discovery and local credential-source integration for the
bundled agent runtime.

Run it through the application's admitted agent setup. The `--metadata` and
`--models` modes provide discovery; ordinary operation uses ACP over stdio. The
host supplies session workspaces, provider configuration and tool permissions.
This helper has a separate Cargo workspace from the storage application.

See [local compute](../../docs/local-compute.md) and the
[agent execution manual](../../docs/agent-execution.md).
