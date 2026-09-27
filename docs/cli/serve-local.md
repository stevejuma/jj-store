# Serve a local native repository service

`jj store serve-local --server-config PATH` starts the same foreground service
as `jj-store run` and requires a literal loopback listener. Storage,
credentials, resource limits and shutdown behavior come from the server
configuration.

This command does not install a daemon or start RustFS. Use
`jj-store config example` for the template and `jj-store help -k config` for
configuration help.

On Unix the command hands off to the paired `jj-store` executable in the same
build directory, preserving foreground process ownership and graceful shutdown.
Other platforms should run `jj-store run --loopback-only` directly until their
handoff behavior is qualified.
