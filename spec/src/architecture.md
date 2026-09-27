# Architecture

Crates are added as milestones begin, not as empty placeholders. Native codecs
initially live with JJ integration. The server uses in-process stores rather
than calling its own RPC. Local indexes are derived and rebuildable. The source
repo is colocated for development; product storage has no Git carrier.

## Requirements

| ID           | Requirement                                                                                                                                                                     |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ARCH-001 | The repository MUST provide a Rust workspace with independently owned native storage, publication, JJ integration, services, transports, adapters, and presentation boundaries. |
| JST-ARCH-002 | Model, object storage, and WAL MUST NOT depend on UI, Tantivy, Git transport, SQL, Nostr, or agent runtimes; upstream JJ integration MUST stay behind owned adapters.           |
| JST-ARCH-003 | First release MUST serve new native repositories locally and on an authenticated shared server using the same storage format, with no required Git, SQL, or relay authority.    |

Storage harness self-tests are part of normal validation. `pnpm test:rustfs`
executes only isolated disposable Docker probes and writes a scratch report. An
approved immutable report is retained with its exact image/topology.

## M0 extension qualification

M0 introduces only CLI and JJ compatibility crates. The upstream example is a
Git-backed test executable; it does not advertise a production backend. Both JJ
crates now resolve together through the committed Cargo lockfile.

## Object foundation

The model crate owns typed IDs, object kinds and envelope framing without
importing JJ. The JJ integration crate owns native schemas and identity checks.
Version 1 metadata is bounded independently of the streaming file transport.

The object crate owns the S3 transport and deterministic fault store. It depends
on the model and transport libraries without importing JJ. Upload preparation
uses bounded temporary disk space rather than buffering unbounded file blobs.

The WAL crate starts with a storage-independent bounded dependency walker. Its
source interface returns owned keys and verified descriptors; the JJ crate
implements native edge enumeration. No native JJ type crosses into the WAL.

The WAL crate owns immutable publication records, persistent radix catalogs,
manifest CAS, durable receipts, replay and checkpoints. Its catalogs depend only
on owned model types and the object transport. Native dependency enumeration
continues to enter through the verified-source interface.

The service crate owns repository/operator token policies and request-bound
authorization. Its publication guard uses owned WAL snapshots/settings and
requires an explicit native semantic validator, keeping authentication shared
between future HTTP and RPC adapters.

The development toolchain includes rustfmt with a tracked edition-2024 policy.
Rust formatting is part of the shared formatting commands, normal checks and
canonical specification checks.
