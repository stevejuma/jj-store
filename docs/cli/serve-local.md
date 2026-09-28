# Serve a local native repository service

`jj store serve-local --server-config PATH` starts the same foreground service
as `jj-store run` and requires a literal loopback listener. Storage, resource
limits and shutdown behavior come from the generated local configuration.

Create that configuration with `jj-store init --local --state-dir PATH`; it
starts the pinned embedded RustFS helper and reuses its explicit volume on
restart. No daemon, Docker service or repository credential is required.
`config example` documents deferred shared-deployment fields and is not the
local setup path.

On Unix the command hands off to the paired `jj-store` executable in the same
build directory, preserving foreground process ownership and graceful shutdown.
Other platforms should run `jj-store run --loopback-only` directly until their
handoff behavior is qualified.
