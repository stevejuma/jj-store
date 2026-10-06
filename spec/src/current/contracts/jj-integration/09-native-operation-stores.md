# Native operation stores

The native OpStore persists complete owned view/operation codecs and exposes
virtual roots from the pinned profile. A request-bound authority interface is
mandatory for read/write authorization, committed operation-ID enumeration and
head publication. Prefix lookup combines a bounded manifest-selected inventory
with only this session's acknowledged staged operations; it never scans bucket
keys or treats arbitrary candidates as committed history. Truncated enumeration
is an error, never a successful partial prefix match.

OpHeadsStore delegates atomic sibling-preserving transitions to the guarded
publication authority. Its optional coordination lock is intentionally empty:
manifest CAS provides correctness. Unknown publication outcomes propagate as
errors, leaving upstream callers unable to claim success. Destructive GC returns
an explicit disabled error. Backend/factory/RPC wiring remains a separate M3
gate.

A full staged-operation budget rejects a new distinct operation, while an
identical acknowledged metadata-upload retry remains idempotent.
