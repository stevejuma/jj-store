# Custom data index lifecycle

JST-CLI-004 and JST-ADAPT-008 govern the optional disposable native read cache.
Format-2 configuration accepts `custom_data_index_persistence`, default true.
Setting false disables disk persistence; bounded in-memory readers remain
available. This infrastructure setting takes effect at server restart.

Persistent files live beneath the configured state directory in
`custom-data-index`, with a private directory, a maximum of 64 generations and
two GiB disk retention. Individual database files are capped at 128 MiB; derived
payloads and retained transition predecessors each have a 64 MiB ceiling.
Publication and recovery never depend on these files. Cache corruption, loss or
failed writes falls back to the same freshly admitted source root. Neither
configuration nor ready metadata grants source or executor authority.

The source binding is private and credential-free. Index writes use one
supervised blocking slot, with immediate same-root scan fallback on pressure; a
canceled caller leaves that slot occupied until its bounded worker exits. No
temporary cache file is a complete generation. Maintenance removes only
index-owned digest-named regular files. Native objects, WAL, fixtures and client
working files are outside this ownership boundary.
