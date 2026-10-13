# Workflows

Native workflow RPC and custom CLI use owned workflow service contracts. Exact
source workspaces use the existing owned WorkspaceDriver and CLI composition;
the server never imports upstream JJ. See JST-WF-005 and JST-WF-006.

E-WORKFLOW-SETTINGS-001 adds one shared reloadable host-admission owner across
native RPC and HTTP. Execution admission is checked through publication, while
reads and ordinary definition mutations retain their existing authority. Native
bindings, source pins, request identities and version-1 workflow RPC documents
are unchanged. The CLI supplies the optional managed runtime to server-owned
configuration interfaces; core service/storage do not import the CLI runtime.
