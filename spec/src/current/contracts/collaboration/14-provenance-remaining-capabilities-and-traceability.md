# Provenance, remaining capabilities and traceability

Opted-in client scopes separate preexisting dirty baselines, checkpoint in
failure and finally paths and retain exact JJ operations/commits. Immutable
intents precede operations; later evidence never changes published hashes.
File-only recovery reports interval/reported evidence rather than exclusive
authorship. Owned producers share canonical registration/binding/append/scope
contracts and explicit optional Thread binding. Full runner dispatch is
separately qualified.

| Original v4 behavior or review finding                                                                           | Requirements                                                               | Gate                                      |
| ---------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ----------------------------------------- |
| Sections 5/12: discovery, active backfill, reader recovery and hook/file enrichment                              | JST-COLLAB-017,JST-COLLAB-018,JST-COLLAB-019,JST-COLLAB-020,JST-COLLAB-030 | M18.2                                     |
| Sections 10.6/18: omitted/unsupported records block drain                                                        | JST-COLLAB-021                                                             | M18.1                                     |
| Section 7.3: cross-batch tool field loss/terminal regression                                                     | JST-COLLAB-022                                                             | M18.1                                     |
| Section 10.2: full-prefix transcript append                                                                      | JST-COLLAB-023                                                             | M18.1                                     |
| Sections 7/25: blocks/plans/children/compaction, exact message/plan reviews and renormalization/replay           | JST-COLLAB-024                                                             | M18.3                                     |
| Sections 8/9/13: automatic provenance and owned producer scopes                                                  | JST-COLLAB-025                                                             | M18.4                                     |
| Section 12.7: real harness edit/UI/restart qualification                                                         | JST-COLLAB-026                                                             | M18.5                                     |
| Sections 15/16/25: exact inspection, search, immutable work requests, feedback addressing and safe comment input | JST-COLLAB-027,JST-COLLAB-030                                              | M18.6                                     |
| Sections 17/25: collaboration archive and portable carriers                                                      | JST-COLLAB-028                                                             | M18.7 and existing M13 portable gates     |
| Sections 11/19/25: home/temp path privacy, target permissions, current revocation and purge                      | JST-COLLAB-029                                                             | Local privacy M18.2; broader policy M18.7 |

M18.5 remains the complete multi-harness capture release boundary, now Deferred.
M18.8 owns the narrower usable Codex preview and does not depend on M18.3–M18.7.
The aggregate cannot be Done while deferred requirements remain open. Existing
native copy, M13 and M15 evidence does not qualify broader gates. Official ACP
Rust SDK adoption is not a required outcome; common producer contracts and
execution qualification are.
