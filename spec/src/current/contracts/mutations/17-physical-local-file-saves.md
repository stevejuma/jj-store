# Physical local file saves

The LocalTrusted working-copy facade binds an admitted physical folder and
device generation. Reads and Save do not call a JJ transaction or initialize
metadata. Save accepts only a selected existing regular UTF-8 file, bounded to
256 KiB, and checks its exact fingerprint under serialized browser admission
before replacing it with a flushed private same-filesystem temporary file under
`.jj`. Descriptor-relative no-follow traversal rejects metadata paths and
symlink escape. File mode is retained and the parent is flushed. A stale
selection retains the original file; a transport loss remains uncertain until
explicit readback. No code-command journal or repository receipt is fabricated
for physical Save. Snapshot is independently authorized.

Local Sync barrier capture and bounded waits reuse the existing owned
replication control types. They grant no repository mutation or upstream
configuration authority and cannot silently capture replacement targets.

Storage maintenance binds the current LocalTrusted administrator before reading
or capturing native state. Service-owned typed maintenance calls use coordinator
authority and never accept arbitrary browser manifests. Integrity has no
mutation side effects; checkpoint publication preserves all code, settings and
domain roots from the successful CAS base.

E-CI-BOOTSTRAP-001 retains service working-tree inventory and private agent
workspace fingerprints while normalizing Unix scalar typedef widths for public
CI. Device/inode, link counts and timestamp bit patterns stay unchanged across
GNU/Linux and macOS. This does not grant content reads, weaken descriptor/race
checks, alter Save semantics or change serialized public responses.
