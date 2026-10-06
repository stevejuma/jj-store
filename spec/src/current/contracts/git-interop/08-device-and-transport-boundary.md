# Device and transport boundary

One invocation-owned native factory registry may bind exactly one pending Git
acceptance to its OpHeadsStore. Ordinary snapshot/reconciliation runs before
that binding, and normal JJ transaction finish supplies the resulting operation
and working-copy update. There is no process-global pending request. A no-op
fetch publishes no code operation. Private cache attempts never become native
working repositories or source checkouts.

The local GB2 slice supports one named remote, default selection and literal or
single-star bookmark filters. Transport captures a bounded complete heads/tags
namespace in one fetch and filters its completed refs before native conversion.
This proves literal absence without an earlier advertisement race; omitted
bookmark refs remain untouched. Commit tags retain original target and peeled
commit identities. Unchanged refs do not introduce visible heads or run a merge,
and a repeated unchanged observation publishes neither code nor bridge state.
This local slice does not qualify code-origin activation or push.
