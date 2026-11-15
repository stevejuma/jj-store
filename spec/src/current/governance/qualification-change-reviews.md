# Qualification change reviews

E-CI-AFFECTED-001 adds conservative Cargo-based standard selection and
source/package/inventory validation to JST-ARCH-005. Invalid graphs, incomplete
partitions and unexpected skips fail closed. PR caches stay in GitHub's isolated
scope; arbitrary qualification sources cannot write shared-main caches.
JST-GOV-003/004/011 retain same-change evidence and M30 review. No product API,
publication authority, generated automation exception or heavy gate is changed.

E-CI-CACHE-TRIAL-001 adds a bounded, explicitly dispatched JST-ARCH-005 cache
experiment. It leaves routine protected checks and manual heavy qualification
unchanged. Three warm samples per backend, exact inventories and passing tests
precede the measured adoption decision; no test-pass cache is introduced.

The read-only native SSH trust-file profile retains E-VAULT-BACKUP-027 as a
separate transport receipt. Loopback proof cannot promote M34.5 beyond Partial
or substitute for host/provider and cold application acceptance.
