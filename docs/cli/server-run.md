# Run the foreground server

Start with `jj-store run --config PATH`. The listening event identifies the
bound address; storage readiness is checked separately with `doctor` against a
known repository. `--format json` emits a plain JSON event without a pager.

Ctrl-C and termination request graceful shutdown. After five seconds a stuck
request is canceled; clients must treat interrupted publications as unknown
outcomes and retry with their original identities. A second process using the
same state directory fails explicitly. Stop the old process before restarting.

Desktop hosts can opt into `--desktop-supervised` with a pipe on stdin. EOF
requests graceful shutdown. This mode requires strict LocalTrusted embedded
storage and accepts launch-only `--desktop-bind`, `--desktop-web-root` and
`--desktop-rustfs-executable` overrides. It leaves stored configuration
unchanged. Overrides require supervision; an ordinary terminal is not an owner
pipe.
