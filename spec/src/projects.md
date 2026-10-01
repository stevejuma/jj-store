# Native projects and tasks

WorkStore is a repository-owned application domain alongside AgentStore and
ReviewStore. One project home owns metadata, tasks and discussion history. A
home may contain several projects and may also contain ordinary code.

## Requirements

| ID           | Requirement                                                                                                                                                                                                                     |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-WORK-001 | Projects MUST have stable identities and link repositories by stable RepositoryId. Retired links preserve existing task targets; new targets require active membership.                                                         |
| JST-WORK-002 | Tasks MUST have title, body, optional linked repository, optional same-project parent and open, in_progress, blocked or done state. Parent cycles and cross-project parents MUST be rejected.                                   |
| JST-WORK-003 | Comments MUST support same-task threaded replies, versioned edits and tombstones with authenticated actors and immutable history.                                                                                               |
| JST-WORK-004 | WorkStore MUST use typed repository-bound immutable records, bounded persistent radix catalogs, an independent manifest root/version, manifest CAS, WAL and durable request receipts. Unselected objects MUST remain invisible. |
| JST-WORK-005 | Reads MUST pin the selected WorkStore root. Mutations MUST validate exact entity and project versions; identical retries and reconstructed-service status MUST return the original durable receipt.                             |
| JST-WORK-006 | Home repository read/write permissions MUST govern metadata; linked-code reads MUST separately authorize the target repository. CLI, RPC and HTTP MUST share owned service semantics.                                           |
| JST-WORK-007 | Cold recovery and manifest-last native copy MUST verify every selected and historical WorkStore closure. Unsupported origin replication MUST fail closed for a WorkStore home while retaining its existing origin ledger.       |
| JST-WORK-008 | CLI and browser MUST expose project links, task state/hierarchy and comments. SSE announces freshness; explicit refresh changes displayed snapshots. Unknown command outcomes retain the request identity.                      |

## Initial profile

The native profile below remains manifest/WAL backed. M13's separate local
Git-backed profile uses the same `WorkStateV1` reducer, semantic validation and
limits through the owned LocalRepository service. It stores an explicitly
versioned canonical local WorkStore snapshot and durable receipts through the
companion Git adapter. Native records and digests are not reinterpreted.

First publication non-destructively upgrades a collaboration-enabled format-2
home to format 3, preserving code heads, code version, sibling roots, settings,
receipts and history. Existing format-1 and format-2 bytes remain readable.
Projects, tasks and comments use distinct 16-byte stable IDs. Each selected
version has its own repository-bound immutable digest. Titles admit at most 512
UTF-8 bytes, bodies at most 64 KiB, project membership at most 256 repositories,
and each catalog at most 4096 entities. Complete reads and closure audits also
enforce an aggregate eight MiB body limit. Exceeding admission fails explicitly
and never produces a truncated accepted state.

Home writers may edit task metadata and comments; authenticated creation and
last-edit actors remain separate facts. Comments retain tombstones and reply
identities. All historical selected versions remain reachable through the
manifest-selected WAL, including prior bodies. A tombstone removes the current
body from normal discussion presentation, without rewriting history.

The initial WorkStore profile is local service backed. An upstream hub is
optional for native repositories, but WorkStore format 3 freezes the M10 origin
protocol until a WorkStore reducer is implemented and qualified. Native creation
already initializes an origin ledger without an upstream; the format upgrade
retains that ledger unchanged. New format-3 publications stay in the
authoritative local WAL. Negotiation, feeds, offers, origin closure, imports and
replication seeding fail explicitly for a WorkStore home. Native copy includes
WorkStore; copy is not origin replication. No Git refs, stock-JJ mirroring,
server-free CLI storage, relay synchronization, custom roles, assignments or
controller dispatch are introduced.

## Acceptance

The focused native gate exercises two linked repositories, cross-repository
parent/child tasks, cycle and stale-version rejection, link retirement, threaded
comment edits/tombstones, pinned reads, receipt replay after reconstruction,
checkpointing, recovery and manifest-last copy. Fake storage and real RustFS
qualification are recorded separately. Transport gates additionally require home
authorization, target-code authorization, executed HTTP contract capture, CLI
publication over local RPC and unknown-outcome status. Browser gates cover
project/task selection, state and hierarchy, comment interactions, explicit
refresh, accessibility and a cold startup. Visual baselines remain pending.

## Paged service reads

A project collection page selects at most 256 projects. A project query returns
that project plus a page of its tasks; a task query returns its project,
selected task and a page of comments. Pages enforce a four MiB body budget as
well as the record count. Opaque random continuations retain the selected root
and bind the exact repository, epoch, filters and page size. The process
registry bounds snapshots, memory, concurrency and five-minute lifetime. Every
resumed page reauthorizes; restart, expiry and resource rejection are explicit.
GET never stages empty roots. Linked-code access uses a fresh target repository
session.

Browser tasks and discussion order loaded parent/reply branches and cap visual
indentation. Missing parents from another page retain their exact identity and a
visible fragment rather than inventing another relationship. Cold RustFS
qualification covers separate source/destination stores, historical WorkStore
closures after checkpointing, retained origin ledger, local readiness and
durable service receipts after both stores restart.
