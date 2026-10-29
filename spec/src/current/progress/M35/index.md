# M35 persistent attachments and editable thread files

The user agreed the scope on 2026-10-23 and requested saving it for later
implementation on 2026-10-24. **All implementation is Planned.**
[Implementation plan](../../../../../docs/maintenance/thread-files-implementation-plan.md)
and
[owning contract](../../contracts/collaboration/45-thread-files-and-attachments.md)
define the agreed behavior. E-THREAD-FILES-PLAN-001 is documentation evidence;
it establishes no storage, agent or browser acceptance.

| ID    | Status  | Depends                 | Requirements                                 | Gate                                                                                                                    | Next                                                                            | Evidence                |
| ----- | ------- | ----------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ----------------------- |
| M35   | Planned | M15.2,M15.3,M24         | JST-COLLAB-037,JST-COLLAB-038,JST-COLLAB-039 | Persistent exact attachments and editable files in standalone chats and Threads across local/native profiles            | Start the M35.1 storage regression when implementation is requested             | E-THREAD-FILES-PLAN-001 |
| M35.1 | Planned | M15.2                   | JST-COLLAB-037                               | Canonical file catalog/revisions and complete native/local content retention with legacy-byte compatibility             | Prove bounded content retention independently of eager metadata materialization | E-THREAD-FILES-PLAN-001 |
| M35.2 | Planned | M35.1,M15.3             | JST-COLLAB-037,JST-COLLAB-038                | Authorized staging, reads, version-checked writes and receipt-backed HTTP/CLI recovery                                  | Implement shared service and generated contracts after storage proof            | E-THREAD-FILES-PLAN-001 |
| M35.3 | Planned | M35.2,M24               | JST-COLLAB-039                               | Atomic exact message references and frozen agent file context with real CLI access                                      | Qualify attachment-only sends and publication-before-dispatch recovery          | E-THREAD-FILES-PLAN-001 |
| M35.4 | Planned | M35.2,M35.3             | JST-COLLAB-038,JST-COLLAB-039                | Reviewed shared Files UI, composer uploads and explicit text editing with history and draft-safe conflicts              | Review focused Storybook states before live adoption                            | E-THREAD-FILES-PLAN-001 |
| M35.5 | Planned | M35.1,M35.2,M35.3,M35.4 | JST-COLLAB-037,JST-COLLAB-038,JST-COLLAB-039 | Cold local/native browser, original-byte downloads, Git GC, real-RustFS recovery/copy and installed-adapter file access | Run original complete gates only after implementation                           | E-THREAD-FILES-PLAN-001 |

## Milestone review

M35 and its five implementation slices remain Planned. M15 and M24 remain In
progress at their existing independent gates; this plan neither promotes them
nor adds file support to their recorded acceptance. Implementation targets the
already admitted storage profiles and retains all origin/reducer refusals.

Static attachments are limited to 20 MiB each, 10 files and 100 MiB per message;
editable Markdown/UTF-8 text initially uses the existing 256 KiB editor limit.
Explicit Save creates versions; message history pins the original revision. New
metadata push/fetch, portable export/import, folders, office editing, CRDTs and
automatic code-repository file linking remain deferred. Plan saving changes no
runtime, generated wire schema, persistent storage format or active service.
<!-- generated-navigation -->

## Pages
