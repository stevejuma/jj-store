# Local compute resources

The application service uses Lapis AI clients to supervise its `lapis-compute`,
`lapis-agent` and `lapis-agent-tools` helpers. Lapis AI owns the maintained
MeshLLM implementation; the application owns model lifecycle admission,
settings, identity custody, data locations and durable operation receipts. ACP
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

The server and desktop link the sibling `lapis-ai` crates using versioned Cargo
path dependencies. Development builds use that working tree and stage all four
helpers from it. CI materializes a recorded source revision at the same sibling
path. A future crates.io switch removes the path selectors; compute remains a
verified native helper, without a Mesh dependency in storage/model/WAL.

Compute commands and ordered events have separate shared client handles. Stop
and progress remain responsive during active operations. The native-only
connection credential supplies local or explicitly selected paired inference;
credentials never enter browser DTOs. Existing compute directories, installed
model receipts, machine identity and peer invitations are retained.
