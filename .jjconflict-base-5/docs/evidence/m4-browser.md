# M4 browser read acceptance

The browser consumes generated native read contracts through an authenticated,
bounded same-origin client. Lossless numeric parsing preserves native integer
widths. Commit, operation and evolution views render declared native edges;
files, metadata, differences and conflict terms remain text. Query retries and
continuations retain captured inputs. Display bounds are 1000 graph nodes, 4096
edges and 256 KiB per preview.

`web/tests/client.test.ts` checks large native integers, complete SSE frame
resumption, token transport/revocation, response limits, empty file ranges and
native graph layout/bounds. `web/tests/browser/reads.spec.ts` runs cold
production assets under the actual Rust static router and CSP. It captures fresh
native HTTP responses first, then intercepts browser API requests with those
samples and explicit race/error variants. This separates browser interaction
evidence from native service/storage correctness.

The browser suite covers three graph views, safe hostile source text,
file/difference inspection, multiple heads, in-flight disconnect and revocation,
mobile overflow, axe accessibility, snapshot/query continuation, explicit index
preparation, SSE resume/reset/refresh and competing file selections. Scrollable
text regions are labeled and keyboard-focusable. A narrowly documented Svelte
lint exception permits that accessibility pattern; axe remains enforced.

The cold executable TLS test loads a disposable bundle through the real server
configuration. It checks assets, CSP, unauthorized API access and the
distinction between ordinary HTTP 404 and native gRPC UNIMPLEMENTED. Asset unit
tests reject escapes, missing or duplicate files, oversized files and symlinks,
and prove that requests use preloaded bytes rather than filesystem paths.

OpenAPI generation normalizes reference siblings into equivalent allOf schemas
so generated TypeScript retains the operation-diff discriminator. The consumer
fixture narrows every status; schemas and generated types remain drift-checked.

The original browser launch failures, fetch receiver error, locator correction,
keyboard-scroll accessibility defect, shared HTTP fallback defect, reset refresh
and empty-file regressions are recorded with actual commands in E-M4-017 of the
single progress ledger. The same entry records aggregate checks and remaining
deployment gates. No intercepted browser request is counted as a real RustFS
browser pass. The stopped shared Docker engine still prevents new RustFS/SSE
qualification until its restart is authorized. Typed mutations and search remain
M5/M6 work. Screenshot review is visual-pending; no baseline was approved.
