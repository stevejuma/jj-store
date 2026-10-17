# jj-store-compute

Application-owned local compute process for model lifecycle, inference and
admitted peer sharing. It exposes JSON control over stdio and authenticated
inference through a loopback gateway, using the pinned Mesh-LLM runtime.

Start it through the application's compute lifecycle so the host supplies its
private identity, authentication and model policy. Keep this independent Cargo
workspace separate from repository storage and agent-session persistence.

See [local compute](../../docs/local-compute.md) and
[process lifecycle](../../docs/site/technical/lifecycle.md).
