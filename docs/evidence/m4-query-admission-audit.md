# Native metadata admission qualification

This slice addresses JST-API-004 and JST-JJ-005. It applies request policy
before native metadata allocation without changing persistence encodings or IDs.

Three actual service/native regressions initially accepted 1,025 commit merge
terms, 1,025 reference merge terms, and a 50,001-entry tree. All three now
reject with Limit. The fixtures first persist, read back and publish the same
values; the query policy does not narrow lossless native storage. A separate
acceptance case proves node counters reset per request and do not prevent
bounded binary streaming. Four scanner unit cases cover every persisted
merge/edge field, limits at and above boundaries, cumulative accounting, escaped
keys, malformed JSON, envelope corruption, depth and cancellation.

Metadata is buffered under the existing 16 MiB object cap, envelope-verified,
and scanned without allocating collections. The scan counts JSON values and map
keys before DTO/native decoding. Escaped strings may use bounded parser scratch.
The normal codec still verifies canonical structure and native identity. Native
file blobs bypass the JSON scan and retain verified private disk spooling.

## Measured accepted inputs and rejections

[Graph evidence](m4-query-metadata-budget.json) reruns the release 50,000-commit
linear/wide scenarios. Eight queries and four negative admission cases pass in
5.88 seconds elapsed, 5.16 CPU seconds and 278,675,456 peak RSS bytes. The
largest case scans 2,221,020 metadata nodes within the 4,000,000-node default.

[Content evidence](m4-content-shape-budget.json) records eight cases in a
separate release process. It passes in 7.70 seconds elapsed, 7.36 CPU seconds
and 111,181,824 peak RSS bytes. The accepted dense tree has 50,000 entries; the
accepted merge combines 65 terms over trees with 5,000 entries, preserving 65
terms for each returned path. Both return exactly 500 rows. Larger trees/merges
and insufficient metadata budgets reject explicitly.

The file scenario verifies a complete 256 MiB object and returns all bytes in
chunks no larger than 64 KiB. The 16-byte tail-range request independently
verifies the same full object before returning bytes. A blob one byte beyond the
configured cap rejects. The fixture stores large file envelopes on disk, so the
measurement includes verification, chunk streaming and temporary spools without
retaining the source blob in memory.

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification
python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/m4-shape-graph-budget.json
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example content_qualification
python3 scripts/qualify-query-resources.py --scenario content --binary target/release/examples/content_qualification --output .scratch/m4-shape-content-budget.json
```

## Remaining qualification

These fixtures use constant permission and immutable in-process storage. They do
not measure service authorization, network/RustFS latency, HTTP or browser cost.
Cumulative node/byte admission is a structural bound, not a precise allocator
reservation or an operating-system CPU limit. The whole-process harness has
independent 1 GiB RSS, 60 CPU-second and 180 elapsed-second qualification
ceilings.

Derived merges from native revset parent traversal, file-content predicates,
recursive tree amplification and aggregate workers still need qualification.
Long cold-index rebuild tasks/status remain open. M4 stays in progress; this
report does not complete shared deployment or release gates.
