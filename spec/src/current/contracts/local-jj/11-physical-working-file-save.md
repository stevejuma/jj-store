# Physical working-file Save

Explicit Working copy editing reads the registered physical folder independently
from the displayed operation. Its selection binds repository, binding
generation, physical workspace identity, relative path and a fingerprint of file
identity, mode, timestamps and exact bytes. Existing regular UTF-8 files up to
256 KiB are qualified; traversal, metadata components, symlink traversal,
hard-linked files, special files and unsupported content fail closed. Save
revalidates the selection immediately before atomic, mode-preserving
replacement. It never snapshots files, publishes JJ heads or changes the
companion ref. Snapshot remains a separate explicit action. File drafts survive
stale selections and uncertain responses; readback can confirm the current file
matches the draft, never a JJ receipt. Concurrent browser saves serialize.
Arbitrary external filesystem writers do not participate in a filesystem CAS;
final fingerprint checking detects prior changes but cannot guarantee exclusion
of a writer racing the atomic rename.
