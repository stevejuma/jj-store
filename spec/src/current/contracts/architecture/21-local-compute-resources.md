# Local compute resources

The application service supervises a separate compute helper and local model
agent. MeshLLM runtime and model management remain application concerns. ACP
retains only trusted launch, stdio protocol, session and process ownership; it
receives fixed coding-tool definitions from trusted configuration. Storage,
model and WAL remain independent of controller and inference dependencies.
Distribution resources pair exact executable versions and native-runtime ABI
artifacts. Third-party source attribution and artifact pins are retained with
the owning integration. M25 qualifies macOS ARM64 desktop and Linux CPU server
independently.

M25 provider selection is a service-owned data catalog. The bundled local-agent
entrypoint owns bounded no-prompt discovery using its existing provider
adapters; server supervision owns effective configuration, credential custody,
cache isolation and cancellation. Native storage/model/WAL dependencies remain
unchanged.

Runtime configuration reload retains listener/storage ownership and agent
supervisors. Configuration owners expose independent drain/replace boundaries
for capture, replication and search; protection/limits share current runtime
policy. Compute and agent process changes respect active work and keep captured
configuration until their supported replacement boundary. Effective reports
reflect loaded generations rather than frozen startup copies.
