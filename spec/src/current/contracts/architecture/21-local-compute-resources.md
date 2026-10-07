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
