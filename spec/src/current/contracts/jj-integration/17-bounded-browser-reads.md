# Bounded browser reads

Reference lookup exposes native bookmark merge terms and visible changes without
inventing a default bookmark or deleted history. Recursive path reads stop after
the requested window and report unavailable last-change metadata when ancestry
or work budgets prevent exact first-parent attribution. Operation grouping uses
recorded predecessors first, preserves ambiguous lineage and requires explicit
parent selection for merge operations. Diff summaries retain exact before/after
terms, copy and rename sources, conflicts and binary/oversized reasons; selected
file content remains a separate verified read. These read-only queries cannot
stage objects, reconcile heads or publish repository state.

Native RPC V1 snapshot messages include the repository code-state version in
addition to epoch, global publication position and heads. Administration clients
round-trip the field so collaboration-only WAL activity does not make a strict
native policy or code request stale. A missing field decodes as zero for
pre-feature canonical snapshots; new current snapshots always return the
selected value.

Native RPC V1 also exposes bounded ReviewStore list, detail, command and status
methods for the bound repository. Wire messages contain repository and stable
identity bytes plus bounded canonical JSON encoded from service-owned models;
the RPC layer does not import the review model in production or expose WAL
coordinators. LocalTrusted calls retain owner-peer verification and
authenticated calls retain bearer policy. Cursor pins, command authorization,
receipt replay and outcome classification remain service responsibilities.
