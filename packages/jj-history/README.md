# @lapismd/jj-history

A headless typed client for operation-pinned history from
`tauri-plugin-jj-store`. The native host admits the vault, source paths, private
cache and caller window. The client only receives the resulting opaque vault
scope and handles.

```ts
import { tauriHistory } from "@lapismd/jj-history/tauri";
import { invoke } from "@tauri-apps/api/core";

const history = tauriHistory(admittedVaultToken, invoke);
const heads = await history.heads();
const handle = await history.open(heads.heads[0]);
try {
  const status = await history.status(handle);
  // Wait through the application's own lifecycle while status is preparing.
  if (status.state === "ready") {
    const result = await history.read(handle, { kind: "query_info" });
  }
} finally {
  await history.close(handle);
}
```

Use `HistoryClient` from the main entry with an injected `HistoryTransport` for
other text transports. Neither entry imports Tauri or chooses native policy.
Errors propagate without retries; applications own cancellation and closure.
Retain failed close handles until the native host confirms their release.

Types and runtime validation derive from the native Rust schema. Integer values
within JavaScript's safe range decode as numbers; larger values decode as
`bigint`. Supply large input integers as `bigint`, never rounded numbers.
Request and reply envelopes are bounded UTF-8 JSON text, version 1. Do not parse
their inner payloads with ordinary `JSON.parse`.

See the
[local repository manual](https://stevejuma.github.io/jj-store/technical/local/)
for host admission, operation pins and platform qualification. Source and
archive checks do not establish a released package or qualify a downstream
application.
