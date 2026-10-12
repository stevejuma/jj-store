# Remote execution authority

JST-API-011 retains JST-MUT-001 semantic publication policy. The service exposes
owned connection and remote-execution interfaces; inference/controller transport
does not enter storage, model or WAL. Native repository mutations and
publication use the initiating caller's current authority, never a LocalTrusted
administrator fallback. Bound read/stream continuation and receipt recovery also
reauthorize.
