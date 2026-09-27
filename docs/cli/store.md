Manage native JJ repositories over authenticated RPC.

Use `jj store init` for a new repository, or `jj store attach` to create a
physical workspace for an existing repository. Ordinary JJ commands then read
and publish the same native history. Connection metadata contains no
credentials. Use `jj store login` or inject `JJ_STORE_TOKEN`.

Use `jj store help -k commands` for the command guide. Server creation and
administration are separate from physical workspace attachment.
