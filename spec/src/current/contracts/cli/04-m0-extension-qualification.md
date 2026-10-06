# M0 extension qualification

The Apache-2.0 upstream example compiles at the pinned release and is exercised
through isolated init/describe/log/op-log/evolog commands. Its Git backend is
reference-only; native commands and presentation coverage remain M3.

The native library lifecycle fixture covers the stores used by the future custom
CLI, including independent local caches, repository attachment, transactions and
reopening. It is not executable command-presentation or remote CLI evidence.

The CLI integration library binds remote RPC authority and immutable object I/O
to the existing native factories. Loopback network lifecycle tests qualify this
binding separately from the future executable command presentation matrix.
