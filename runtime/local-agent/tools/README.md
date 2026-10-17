# jj-store-agent-tools

Bounded MCP tools for reading, searching and editing workspace files, plus
command execution. The managed ACP agent obtains host permission before invoking
these tools in its selected session workspace.

Compose this executable as the managed agent's stdio tool server. The session
working directory supplies the file root; requests carry typed tool arguments
and bounded execution deadlines. Tool execution is separate from repository
publication and capture receipts.

See the [agent execution manual](../../../docs/agent-execution.md) and
[local compute](../../../docs/local-compute.md).
