# M4.1 authenticated native reads

The enclosing Jujutsu change composes application reads on the production
listener. M4.1 concerns pinned native reads; M4 as a whole also requires the
browser, generated contracts and committed events in M4.2.

## Gate evidence

| Contract                                       | Evidence                                                                                                                                                                                                                                                                                            |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fixed native operation and no GET publications | `http_reads_match_native_snapshots_objects_graphs_and_differences_without_publication`; real TLS fixture compares HTTP and ordinary CLI commit IDs at the same operation and checks unchanged manifest snapshot after reads.                                                                        |
| Three graph types and complete metadata        | HTTP success fixture covers log, operations, evolution, native objects, commit and operation detail; existing native graph/content suites retain semantic and codec parity.                                                                                                                         |
| Signed coherent continuations                  | `http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication` retains the original snapshot after publication, rejects changed queries and requires explicit selection of concurrent heads. Existing graph/directory cursor fixtures also cover expiry, restart and epoch changes. |
| Verified media                                 | Full verification precedes streamed bytes; closed/open/suffix ranges, malformed/multiple ranges, one-time spool selection and current revocation are exercised by the HTTP fixtures. Real RustFS fixture reads binary ranges and recovers the same file after cache loss.                           |
| Admission and lifetime                         | HTTP fixtures cover URI/JSON limits, unframed request content, unpolled response expiry and shared native slot release. Listener fixtures cover retained connection capacity and idle activity/deadlines.                                                                                           |
| Same authentication as RPC                     | Repository session binding precedes source reads. Missing/duplicate credentials, wrong repository/origin, cookie-only credentials and revocation fail closed. TLS tests cover HTTP/1.1, HTTP/2, stalled handshake expiry, untrusted certificates, hostname mismatch and plaintext rejection.        |
| Cache-independent recovery                     | [Pinned RustFS report](m4-http-rustfs.json): all 11 harness cases passed, including actual server/CLI processes, TLS HTTP reads, server/RustFS kill and restart, deleted server state/native indexes, retained local edits and token revocation.                                                    |

Named HTTP fixtures are in
`crates/jj-store-cli/tests/native_lifecycle/http_reads.rs`; process fixtures are
in `crates/jj-store-cli/tests/server_executable.rs`. The single progress ledger
records exact commands, observed pre-implementation failures and full
validation. The deployment report qualifies the pinned single-container RustFS
topology, not distributed storage or multiple serving instances.

## Transport and limits

The server merges Axum application routes and the existing native RPC service on
one configured public URL. Its owned TLS acceptor advertises h2 and HTTP/1.1.
The configured concurrent-request count bounds RPC; HTTP uses that count capped
at 64, with two shared native workers. Separate bounded pools are intentional;
HTTP and RPC do not share one aggregate request permit.

Connections across both transports are capped at four times the configured
request count. At most 32 handshakes proceed concurrently, with a five-second
handshake deadline. Socket inactivity uses the configured request timeout;
connection age is 300 seconds plus a request-timeout grace. HTTP/2 streams per
connection use the configured count and header lists are capped at 16 KiB. The
HTTP layer additionally caps URI, header count/bytes, JSON input/output and
total request/response lifetime. Full process and deployment performance
qualification remains M8a; these tests establish admission behavior.

API errors use fixed categories, with no internal storage errors or credential
bytes. JSON/media disable caching and MIME sniffing. File responses are binary
attachments and expose repository/epoch/operation/publication/revision headers.
Origin checks use the canonical configured public origin. URL credentials and
cookie authentication are unsupported. Bodyless routes verify EOF, including
HTTP/2 DATA without Content-Length. Starting an index task prepares only a
rebuildable cache and does not publish repository history.

## Remaining scope

SSE, generated OpenAPI/TypeScript contracts, the browser, typed mutations and
search remain explicit subsequent gates. Capabilities advertise those as
unavailable. JST-JJ-003's read half is exercised here; its application mutation
transaction/descendant-rewrite half remains part of M5. Whole-requirement status
must not be inferred from this read-gate audit.
