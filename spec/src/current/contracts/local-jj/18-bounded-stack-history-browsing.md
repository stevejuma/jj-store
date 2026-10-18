# Bounded stack history browsing

The physical scan foundation is a private provider input under JST-LOCAL-004 and
JST-LOCAL-005, separate from operation-pinned history and from a displayed
Unstaged assignment. It MUST read only the registered physical workspace, retain
a workspace identity and exact bytes/modes/path fingerprint, and enforce entry,
byte, individual-file, depth, deadline and caller-cancellation limits. The
fingerprint includes untracked and ignored checkout obstacles; ignoring a file
for JJ assignment does not authorize overwriting it. JJ ignore/baseline
projection remains a separate provider step. Metadata components are excluded,
symlinks are fingerprinted as link targets without traversal, and hardlinks or
special/unsafe entries fail explicitly without returning a partial selection.
Two bounded passes MUST agree before returning; concurrent changes fail stale.
This is a revalidated observation, not a filesystem CAS or lock against
arbitrary external writers. Apply/checkout still require foreground JJ
coordination and fresh physical preconditions. No scan writes files, metadata or
operation heads, and a scan alone grants no command or content-selection
capability.

The private baseline reader MUST select the working-copy commit from the named
JJ workspace in the exact authorized operation/view, without opening a latest
repository, initializing an index or snapshotting disk files. It returns bounded
raw-content digests and file/link modes; unsupported conflicts, submodules,
unsafe paths or incomplete objects fail without a partial baseline. A historical
baseline is not proof of the current physical working-copy state. The provider
must separately verify checkout metadata, sparse state and current operation
before enabling execution or projecting the active combination's Unstaged work.

Ignore classification MUST use JJ's upstream matcher with explicitly captured
global and repository-exclude inputs followed by per-directory `.gitignore`
bytes. Nested rules cannot reinclude files inside an ignored parent directory.
Tracked baseline paths remain tracked even when ignored. Physical observations
retain ignored and untracked obstacles; their classification never authorizes
checkout replacement. Missing capture of global/exclude policy cannot silently
be treated as empty policy. Baseline/projection helpers add no transport route,
content-selection token or writable capability by themselves.

Current physical reads MUST capture existing local checkout metadata through
bounded descriptor-relative reads, decode the pinned upstream protobuf schema
without initializing missing state, and prove the selected operation, named
workspace and exact tree terms against the authorized WC baseline. They retain
sparse patterns and must not interpret excluded tracked paths as deletions or
outside-sparse files as candidate additions. Global and repository-exclude
policy must be explicitly captured with bounded bytes, verified absence and a
source fingerprint; failed policy reads cannot become empty rules. Repeated
metadata, policy and operation-head observations must agree. A current read is
still an observation, not a checkout lock or command authority.

The service observation owner MUST hold bounded non-queuing admission for the
complete metadata/scan/comparison read. Canceling the caller cancels its worker
but cannot release scan admission before a blocking worker exits. The current
metadata/policy/head witness before and after the two-pass physical scan must
match, including registered repository and working-copy directory identities.
Authorization is checked throughout the scan and before returning. One overall
deadline covers provider reads, scan and sparse comparison; stale, revoked,
cancelled, unsupported or oversized reads return no partial observation. Missing
checkout metadata is never initialized. Conflict labels remain ordered and must
match the selected WC tree along with its terms. Private observations add no
transport or command capability; active combination assignment remains separate.

The registered observation transport accepts only a version, expected binding
generation and exact operation. It MUST resolve the canonical physical root from
current authorized device registration and revalidate that same binding for the
entire read. Folder paths and workspace names are not client selection inputs.
Its bounded complete response is observation evidence, not a content-selection
token, command preparation or proof of active combination membership. Private
policy bytes and absolute source paths stay outside the response. Native hosted
repositories and Git-only folders do not gain checkout capabilities from this
local route.

Production physical observation ownership MUST bracket each read with fresh
registered current-operation snapshots, retaining an exact generation and one
unambiguous head. These reads do not retarget historical code selections or
reconcile siblings. Caller cancellation and disposal release only the owned
waiter and reject late results. Complete client validation and a successful read
still grant neither content-selection tokens nor active-combination assignment.

The approved M21.2 read slice adds a strict typed stack-history query under
JST-LOCAL-004. It binds an exact head and optional base to the existing
authorized operation and binding generation. With a base, the query returns head
ancestors excluding base ancestors; without one, it returns bounded history for
the chosen head. Responses retain graph parents/edges, conflict/divergence
annotations and bookmark identities. A list position is never a graph parent or
mutation boundary.

The query reuses only the qualified private derived index and existing work,
metadata/output, cancellation and lifetime bounds. A cold reader reports the
existing explicit index preparation condition; it cannot repair a source index
or publish code. Continuations bind the complete original head/base/page query,
operation and binding generation, and reject anchors outside the selected range.
Target selection remains explicit; this read does not assume `main`, initialize
a combination, fetch refs, snapshot physical edits or enable code mutations. Net
stack changed files continue to use exact base/head tree comparisons under the
existing diff query, including revert-to-base.

Trusted status composition MAY retain bounded raw bytes privately to compare the
complete observed tree directly with the recorded working-copy parent. It MUST
apply the qualified ignore/sparse projection, count each changed path once, and
revalidate the complete observation after counting. Recorded and unsnapshotted
deltas MUST NOT be summed: edits can cancel. Text counts use the bounded JJ line
diff; binary, structural or complexity refusals retain a file count with absent
line totals. Merged parents and conflicted baselines remain explicitly
unsupported until qualified. Private bytes MUST NOT enter observation/baseline
wire frames.
