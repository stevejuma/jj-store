---
schema: 1
id: E-M12-010
---

# E-M12-010 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
target/debug/jj --no-pager store capabilities --format=json
```

**Notes**

Development executable reports GB6, exact SHA-1 profile and macOS ARM64 qualification with explicit unqualified platform/forge scope; native Git import/export remains unsupported.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_git_https_uses_approved_ca_and_askpass_without_serializing_secrets
- crates/jj-store-cli/tests/executable_native.rs#native_git_ssh_uses_approved_key_and_strict_host_trust
- crates/jj-store-cli/tests/server_executable/git_bridge.rs#real_rustfs_native_git_external_merge_cycle_survives_cold_restart
- web/tests/real/git-bridge.spec.ts#Git import waits for refresh
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

GB0-GB6 accepted on macOS ARM64 with pinned JJ/Git and SHA-1 bookmark profile. Maintain exact conversion, native fault/replay and cold browser/RustFS regressions. Tag publication, signing/conflict export, native Git import/export, server-free storage, Linux/Windows and external forge deployment qualification remain unsupported or deferred; WorkStore replication remains frozen. Root aggregate, full Rust workspace and unrelated UI suites were out of scope under targeted policy. No installation, service restart, publication, migration or visual baseline approval.
<!-- /evidence-field -->

