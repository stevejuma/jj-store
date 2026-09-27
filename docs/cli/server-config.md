# Server configuration

Use `config example` for the TOML template and `config check --config PATH` to
validate it. Paths are relative to the configuration file. Keep state and
operator policy outside Git/JJ checkouts. Unknown keys and invalid budgets fail.

`JJ_STORE_S3_ACCESS_KEY` and `JJ_STORE_S3_SECRET_KEY` supply server-only storage
credentials. `JJ_STORE_S3_SESSION_TOKEN` supports temporary credentials. These
values are never accepted in TOML. The bucket must already exist.

Shared listeners require TLS certificate and private-key paths. Plain HTTP is
allowed only for an explicitly enabled literal loopback listener and endpoint.
The default request limits are eight concurrent requests, 30 seconds and 32 MiB
per native blob. Configure one serving instance per storage namespace; the local
state lock does not provide distributed fencing.
