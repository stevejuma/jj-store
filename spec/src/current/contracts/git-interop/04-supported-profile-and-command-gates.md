# Supported profile and command gates

Retain JJ 0.45.1 and gix 0.87.1. Initial transport qualification uses Git 2.55.0
and SHA-1. Preserve fetched lightweight and annotated commit tags; tag
publication remains deferred. Unsupported selected refs/options reject the whole
command before external writes. Native init/attach retains its existing
spelling; native Git init/clone does not create a new initialization contract.

GB0 establishes routing and explicit unsupported results only. GB1 qualifies
exact representations and cache loss. GB2 adds remote configuration and fetch.
GB3 adds single-ref push, dry-run and attempt status/recovery. GB4 qualifies
faults, concurrency and code-repository replication. GB5 completes tested
bookmark selectors, remote rename/remove and atomic multi-ref pushes. GB6
qualifies the cold real-RustFS external merge cycle and resource/credential
gates. No stage can advertise an unimplemented later capability.

GB0 through GB6 are qualified on macOS ARM64 with the `jj-git-sha1-0.45.1-v1`
bookmark profile. E-M12-010 records generated-credential HTTPS/SSH transport and
a cold native-to-Git-to-external-merge-to-native cycle using actual persistent
RustFS and the served browser. This is local fixture qualification;
Linux/Windows and external forge deployments remain unqualified. Native Git
import/export, tag publication, signing/conflict export and server-free native
storage remain unsupported. WorkStore replication stays frozen.
