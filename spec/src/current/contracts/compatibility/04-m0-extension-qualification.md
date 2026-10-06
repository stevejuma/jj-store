# M0 extension qualification

Compiled M0 probes live in jj-store-jj compatibility tests and the pinned
upstream CLI example. `scripts/smoke-upstream-cli.py` runs the latter with
isolated configuration and disposable storage. Query probes exercise a LocalSet
with a timeout and row limits; production traversal/memory budgets remain M4.

The second cold RustFS run adds an actual truncated HTTP upload; its object
stays absent.
[M0 cold evidence](../../../../../docs/evidence/m0-rustfs-cold.json) retains all
eight probe outcomes. All M0 prerequisites are now met for this initial
topology.
