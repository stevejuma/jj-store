# LocalTrusted native transport

On supported Unix hosts, a LocalTrusted server exposes native RPC only through
its installation-owned filesystem socket. The socket is mode 0600 and each
accepted connection must report the owning UID before a stable
installation-derived principal is bound. The transport accepts no bearer
metadata in this mode. Repository creation, attachment, reads and publications
still use the existing native factories and guarded service sessions; operation
IDs never become client or installation identities. Authenticated TLS RPC keeps
its existing bearer policy and wire contract.
