---
schema: 1
id: E-INJECTABLE-STORES-001
---

# E-INJECTABLE-STORES-001 details 4

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; rustc 1.99.0 (b940084d7 2026-09-28); jj 0.45.1; Git 2.55.0; Cargo workspace packages 0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Unchanged canonical control-record envelope v1, content digests, native manifests/WAL/receipts, Git companion v1/v2, and current-only Agent descriptor v2. New Rust library interfaces have no new disk or transport encoding.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed foundation slice: dependency-light five-store interfaces, named factories, neutral immutable engines/codecs, and native/service adapter cutover. Existing domain behavior and persistence formats remain unchanged. The broader M32 provider registration, lightweight Git extraction, application factory migration and external-consumer qualification remain later gates; no full-plan completion is claimed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete M32 provider transaction/factory composition and extract the lightweight Git provider, then native provider factories and external-consumer qualification. Root aggregate pnpm check/test/build, unrelated UI/browser/visual lanes, real-RustFS topology/release qualification and public publication were outside this foundation slice.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Adds JST-ARCH-006 and the public library boundary. JST-ARCH-001/002/005
  ownership/release policy and JST-PUB-001/004/005/007/009 publication,
  identities, immutable membership and receipts retain their interpretation.
  JST-LOCAL-001/003 canonical companion bytes, domain behavior and replay remain
  unchanged; complete native/Git provider suites retain the affected semantics.
  JST-OBJ-001 S3 unknown-outcome bookkeeping is unchanged by its equivalent lint
  repair.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: Introduces M32 and M32.1 as In progress with this independently verified
  foundation. M32.2/M32.3/M32.4 remain Planned pending provider extraction,
  native factories and consumer acceptance. Existing M9/M11/M13/M15 and M0
  storage qualifications retain their original statuses and gaps. Interface
  compilation and focused passing results do not complete M32.

```
<!-- /evidence-field -->

