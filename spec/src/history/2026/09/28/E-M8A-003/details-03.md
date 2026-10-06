---
schema: 1
id: E-M8A-003
---

# E-M8A-003 details 3

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; rustfs crate 1.0.0 with default features disabled and source revision d47f54bfb2f39f48bd1adda334bd27e151fe85b8; OpenAPI 3.1; TypeScript generated contracts; Storybook 10.5.7; Playwright 1.63.x.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds V1 bootstrap DTOs for LocalTrusted access mode, public capabilities, storage readiness and installation catalog entries, with reserved unavailable backup-status fields. Adds local format-2 Unix socket connection records whose credential identifier is absent. Configuration format 2 is now the only accepted pre-release server format; the temporary format-1 reader, fixture and migration command are removed. Native object identities, manifest CAS, WAL records and receipts are unchanged. Browser and diagnostic requests send X-JJ-Store-Protocol: 1; LocalTrusted HTTP and Unix RPC carry stable installation-derived principals without bearer credentials. Authenticated/shared serving, external S3, backup/restore, replication, generic-S3 qualification and Linux packaging are explicitly deferred.
<!-- /evidence-field -->

