# CI execution and scheduled validation

E-CI-LATENCY-001 changes execution ordering and scheduled coverage under
JST-ARCH-005 without reducing ordinary selection. Source Storybook requires
same-run source admission and no payload producer. App/Help and static catalog
require their distinct successful producers and complete source/attempt-bound
receipts. All seven UI jobs remain mandatory exactly once. Ordinary shard
reports and commands MUST prove complete disjoint execution before `required`
passes. Missing, failed, canceled and incomplete evidence MUST fail.

Scheduled production MUST be canonical repository/main in `ci-heavy.yml`, with
source matching the scheduled run SHA. Same-run consumers MAY proceed before the
standard gate; external reuse MUST retain that gate. Complete qualification MUST
retain every standard and heavy lane. Independent freshness health and advisory
three-repetition quarantine diagnostics MUST remain distinct from qualification
and publication. Narrower PR selection remains disabled until full hosted
coverage is established. No latency or reliability improvement is claimed from
local tests alone.
