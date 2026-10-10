# Intentional Thread workspace API

JST-API-001 adds LocalTrusted Admin POST `/v1/local/threads`, accepting a
version-1 stable request and either explicit creation or a conditional Codebase
update. Creation provisions the personal metadata home and returns its canonical
home, Thread and committed revision. Codebase updates retain the exact observed
Thread revision. Both paths publish through existing native/local receipts.
Duplicate intent recovers the original receipt; changed intent at the same
identity is refused. Reads and agent startup never create user Threads.

Thread summaries expose optional Codebase membership and internal purpose.
Directories and ordinary search exclude internal owners and annotations; exact
authorized reads retain them. Missing fields preserve legacy canonical bytes.
Conversation sends carry an optional exact Thread Codebase witness, and the
server verifies membership and each repository authorization before dispatch.
Repository membership is context selection and never grants code permission.

Generic browser Thread mutation endpoints refuse internal-owner creation and
Codebase commands; installation-local configuration owns the active-work gate.
The bounded `CreateAnnotation` intent remains a repository-authorized
collaboration operation for browser comment containers, without execution or
Codebase authority. Its exact source discussions follow existing validation.
Send witnesses name the complete observed Thread revision. Unrelated Thread
changes may require an explicit refresh before a new Send, while an admitted
request always retains its original witness for receipt recovery.
