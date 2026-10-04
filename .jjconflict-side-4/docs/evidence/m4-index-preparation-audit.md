# Native index preparation tasks

This change adds a controlled preparation path without raising the hard native
query limits. Graph reads use a 1000-retained-commit inline threshold by
default. Cold reads above it return rebuild-required; warm reads keep their
normal path. The executor builds the pinned index in a cancellable shared worker
and publishes only its disposable local cache receipt. Native repository
authority never changes.

The service accepts an explicit operation and checks current read permission. It
bounds snapshot capture and running jobs (two by default), task retention (32),
and lifetime (300 seconds). Native query and captured-read deadlines can expire
sooner. Running jobs for one repository/epoch/operation/principal coalesce.
Status checks fresh permission and epoch; cancellation additionally requires the
original principal. Random task IDs carry no authority. Completed records retain
owned snapshot/result data without credentials or full inventories.

Dropping the initiating request after acceptance leaves an explicit task.
Cancel, expiry and service teardown signal native cancellation. The task remains
Canceling until its worker exits; synchronous native steps still observe
cooperative checkpoints. A supervisor converts worker panics to an unavailable
outcome instead of retaining a phantom running task. Expired or restarted task
IDs reset explicitly. A completed preparation does not pin local cache files
against eviction.

Cursor rebuild-required errors retain the original complete snapshot even after
a later native publication. Preparation and retry select the same operation; the
cursor continues to enforce its original inventory and publication position.

Named native lifecycle fixtures exercise explicit cold admission,
exact-operation preparation, cache loss, hard budget rejection, worker
cancellation, bounded task retention, expiry without polling, service drop,
valid cross-repository tokens, revocation, restart reset, and cursor
preservation during later writes. Memory storage is distinct from deployment
conformance. The production HTTP listener and browser task presentation remain
separate integration work.

The release resource fixture runs both linear and wide-head histories with
50,000 commits each. It records cold admission and index preparation separately
from prepared/warm reads in
[the process report](m4-index-preparation-budget.json). The test sets the inline
threshold to zero to exercise explicit preparation even when invoked with a
smaller fixture. All hard byte/shape/work/index budgets retain their defaults.
Measurements include fixture construction and local native work; they exclude
service authorization, RustFS/network and browser costs.
