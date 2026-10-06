# Requirements

The local UI development profile separates Vite source serving from the existing
native backend executable. Vite owns the browser loopback port and proxies
versioned API requests to a separate literal-loopback backend port. The existing
fixture remains authoritative and no build is required for UI edits. A
full-stack rebuild/restart remains an explicit development command; production
continues to serve the built asset manifest through Rust.

The CLI has one collaboration input-admission module for both providers. Native
reads and mutations consume only the typed RPC client, while filesystem commands
retain the companion facade. An uncertain RPC command response is reported with
its original request ID rather than silently retried.

| ID           | Requirement                                                                                                                                                                                                                             |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ARCH-001 | The repository MUST provide a Rust workspace with independently owned native storage, publication, JJ integration, services, transports, adapters, and presentation boundaries.                                                         |
| JST-ARCH-002 | Model, object storage, and WAL MUST NOT depend on UI, Tantivy, Git transport, SQL, Nostr, or agent runtimes; upstream JJ integration MUST stay behind owned adapters.                                                                   |
| JST-ARCH-003 | First release MUST serve new native repositories through LocalTrusted loopback HTTP and owner-only Unix RPC backed by persistent embedded RustFS, with no required authentication, Git, SQL, relay, or external object-store authority. |
| JST-ARCH-004 | Browser and catalog source MUST enforce counted-line budgets, keep production independent of fixtures, and separate presentation from API and controller ownership.                                                                     |

Storage harness self-tests are part of normal validation. `pnpm test:rustfs`
executes only isolated disposable Docker probes and writes a scratch report. An
approved immutable report is retained with its exact image/topology.

Development-only WorkStore samples live in the checkout-owned harness. Fresh
fixture setup and `pnpm dev:seed:work` use the production HTTP command boundary;
neither direct object persistence nor catalog fixtures become repository state.
