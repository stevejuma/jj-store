# Initial profile

Browser project listings align creator avatars across rows independently of
count and date widths. Initial collections and task details use busy skeletons
from discovery through their first accepted read, while background refresh
retains displayed data and drafts. Settled empty and failure states remain
explicit; this changes no WorkStore identity, read or command semantics.

Task detail keeps the title header and writable comment composer in fixed page
rows, with metadata, description, subtasks and comments scrolling between them.
Read-only detail retains the fixed header without a composer. Scroll, Activity
collapse and background refresh do not discard drafts or reply/edit context.

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
