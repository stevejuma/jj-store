# Delegated native workspaces

JST-SEC-009/JST-API-011 add a private same-UID Unix socket per caller-owned
conversation. The initiating caller's current repository policy remains the
authority for native reads, workspace publication and receipt recovery. The
process supplies no bearer credential and cannot choose another principal,
repository scope, administration permission or creation authority.

The existing native RPC schema, JJ identities, immutable read pins and
manifest/WAL publication contracts remain unchanged. Materialization uses the
paired custom JJ executable through the delegated endpoint. Revocation closes
workers and prevents later publication; restart retains outcomes without
replaying prompts. No filesystem mirroring or Git transfer is added.
