# M0 extension qualification

M0 probes register a request-scoped OpHeadsStore through StoreFactories,
preserve per-principal publication IDs and exact preconditions across real
loopback gRPC, and load explicitly pinned divergent operations in a bounded
local async task. The RPC fixture is test-only and does not implement durable
publication or policy.
