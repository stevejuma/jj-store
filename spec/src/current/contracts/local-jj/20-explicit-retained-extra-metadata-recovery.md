# Explicit retained extra-metadata recovery

An explicit saved-history recovery MAY restore missing GitBackend extra records
from the original acknowledged retained archive. It MUST retain foreground
coordination and acquire the pinned upstream extra-table lock without unbounded
waiting. Complete existing-record, directory, link, size and archive preflight
precedes source selection. A missing coordination lock may be created during
preflight; it does not select a table record. At most 128 segments, 64 heads, 4
MiB per segment and 16 MiB of total table bytes are admitted. Existing decoded
commit metadata must agree exactly with the archive; conflicting or unsupported
state fails without overwriting an extra entry. Raw Git existence or implicit
import cannot synthesize change IDs.

Recovery MUST preserve every existing extra segment and head byte-for-byte and
add only bounded canonical missing records through synced create-only segments
and head markers. Private upstream materialization verifies the supported table
format before source writes. Revalidate authorization, saved physical binding,
metadata/policy/head witnesses and held directory/lock identities throughout.
Interrupted recovery may leave exact immutable segments or partially selected
missing records; original-request retry verifies those bytes and completes the
remaining archive without a replacement command. Admission and locks stay held
until canceled workers exit.

Decoded source/prepared commits, including change IDs, predecessors, conflict
tree terms and labels, MUST remain identical before acknowledgement. This
recovery changes only explicitly missing extra metadata; it MUST NOT import
refs, select an operation head, recreate a derived index, modify companion state
or write checkout bookkeeping/physical files. Runtime journal authorization,
operation publication and guarded checkout remain separate required gates.
