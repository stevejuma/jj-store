# Native query worker qualification

This evidence covers JST-API-004 and JST-JJ-005 within the production native
query executor. M4 remains in progress in the canonical ledger.

The release fixture encodes 50,000 commits, including the virtual root, in two
shapes: a linear chain and 49,999 sibling heads. It executes cold and warm log
pages, an exact nonmatching subject filter that scans metadata, and operation
history. Additional cases reject excessive commit counts and recursive syntax.
The fixture implements immutable reads only; attempting publication panics.

## Observed failure and repair

The [initial process run](m4-native-query-budget-before.json) exceeded the
independent 180-second timeout, consuming 177.47 CPU seconds. A one-second live
macOS sample (`sample 97217 1`) showed the worker in native revset predicate
iteration, through `pollster::block_on`, `NativeBackend::read_commit`,
`RuntimeContext::run`, and Tokio cooperative-budget polling.

Small bounded-poll tests reproduced exhaustion in both a future and a stream.
The native runtime adapter now polls these futures without Tokio's
scheduler-only cooperative budget. A foreign synchronous executor cannot
replenish that budget. Owned byte/work/deadline/cancellation guards remain
active, and genuine pending I/O still returns Pending. Both exact regressions
pass.

Separate cancellation fixtures first demonstrated detached graph/content jobs
retaining resources until the read timeout. Request-drop guards now cancel
pending authorization and remote bodies. A synchronous test gate proves that
permits and the shared cache lock remain held until the worker actually exits.
Successfully returned verified streams remain usable under their own lifetime.

## Reproduction and results

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification
python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/query-budget.json
```

The [final report](m4-native-query-budget.json) records the binary and Cargo
lock checksums, host, inputs, exact queries, counters and process measurements.
All eight cases and four negative admission checks pass. The recorded whole
process used about 4.74 seconds elapsed, 4.13 CPU seconds and 235 MiB peak RSS,
including encoded fixture objects. Cold log reads took about one second;
complete counters and timings are in the report. Each retained native index used
approximately 5.2 MB of file bytes.

These measurements use the configured 50,000-commit ceiling, 256 MiB cumulative
native reads, 8 MiB output, 1,000,000 work units and 10-second query deadline.
The independent harness ceilings are 1 GiB peak process RSS, 60 CPU seconds and
180 elapsed seconds. Harness ceilings detect qualification failure; they are not
production per-request memory enforcement.

## Shared runtime storage requalification

The [real RustFS rerun](m4-runtime-rustfs.json) passes all 11 harness cases,
including compiled native transport, cold publication recovery and the TLS
server/CLI restart and revocation scenario. This checks the shared runtime
adapter against real pending network I/O. It is separate from the in-memory
query timing above and makes no HTTP/browser performance claim.

## Limits of this evidence

This is an encoded in-memory fixture, with no service authorization, RPC, HTTP,
RustFS or browser latency. The measured native histories have empty file trees;
these numbers do not qualify adversarial merge/tree shape amplification, large
content queries or aggregate server memory. Those admission cases and controlled
long-rebuild task/status handling remain M4 work. Deployment throughput and
whole-server operational budgets remain M8a work. No first-release milestone is
completed by this report alone.
