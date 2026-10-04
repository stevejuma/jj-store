# Native predicate and traversal qualification

This slice addresses JST-API-004 and JST-JJ-005. The canonical ledger records
all baseline failures, fixture repairs, actual commands and wider results.

## Admission and native semantics

File predicates reserve per-file bytes (256 KiB), cumulative materialized bytes
(4 MiB), cumulative lines (1000) and squared line-comparison work within the
query work budget. Counts include repeated materialization at different logical
paths even when immutable file IDs are identical. These limits are separate from
256 MiB streamed content downloads. Native queries use pinned default line-level
merge options; they do not load user merge configuration.

Before a file predicate evaluates, the request scans the selected native index's
commits and mirrors the pinned recursive common-ancestor selection order using
counts. It bounds intermediate commit/tree terms at 65, reserves label strings
including temporary labels, and charges ancestry/simplification work. It does
not construct merges or implement predicate results. The admitted query is still
parsed and evaluated by JJ. Conservative admission can reject a query before
native simplification would cancel terms; it returns Limit, never a partial set.

Implicit native tree reads use the same 64-component / 4096-byte path limits as
explicit content queries. This applies to graph predicates and recursive content
diffs. The default authority hook preserves ordinary CLI/native-session read
permissions and does not change stored data.

Seven acceptance cases cover oversized files/line counts, 34-parent derived
merges, recursive criss-cross expansion, 70-level trees, admitted file and merge
queries, and repeated tree paths. Positive boundaries include a 256 KiB file
with 901 charged lines and sixteen 256 KiB reads totaling exactly 4 MiB. A
seventeenth read fails. Result parity is compared directly against unrestricted
pinned native evaluation for four histories and four predicate expressions,
including criss-cross graphs. All graph/content reads leave publication state
unchanged where asserted.

## Runtime failure and repair

The repeated-file acceptance case initially spun under JJ's synchronous pollster
executor after six other tests had passed. A live sample of the owned test
process identified `matches_diff_from_parent`, `MaterializedFileValue`,
`FileReader`, Tokio's file reader and cooperative-budget polling. The process
was deliberately terminated after diagnosis; this is recorded as a failed run,
not a successful timeout test.

A bounded-poll unit regression reproduced the failure without a wall-clock wait.
Returned native file readers now poll through the same runtime adapter as native
futures and copy streams. This enters the caller-owned reactor and disables the
scheduler-only cooperative budget for that poll. The exact regression passes 512
reads and preserves genuine Pending I/O. The predicate suite subsequently passes
normally.

## Release and real-storage evidence

The [release report](m4-native-predicate-budget.json) records the exact compiled
test executable, binary/lock hashes, host and all seven test names. The final
process passed in 4.10 seconds elapsed, 0.85 CPU seconds and 19,169,280 peak RSS
bytes. The independent qualification ceilings were 180 elapsed seconds, 60 CPU
seconds and 1 GiB RSS. Compilation is excluded from these measurements.

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --release --locked -p jj-store-cli --test native_lifecycle --no-run --message-format=json-render-diagnostics
# Use the native_lifecycle executable from the compiler-artifact record:
python3 scripts/qualify-query-resources.py --scenario predicates --binary <release-test-executable> --output .scratch/predicate-budget.json
```

The [real RustFS rerun](m4-native-reader-rustfs.json) passes all eleven harness
cases after the reader-runtime fix, including native storage transport, cold
publication recovery, and TLS server/CLI restart and token revocation.

## Scope

The release predicate process includes production authenticated service read
sessions, native queries, fixture publication, direct native comparison and
memory storage. It does not measure RPC, RustFS, HTTP or browser performance.
The RustFS report is separate storage/lifecycle evidence. Native synchronous
steps remain cooperatively bounded by input/work admission; these are not OS CPU
or allocation reservations. Shared worker composition and controlled long
cold-index rebuild status remain before HTTP exposure, with deployment load,
security and whole-server limits in M8a. This report does not complete M4.
