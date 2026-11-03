# Acceptance

The focused native gate exercises two linked repositories, cross-repository
parent/child tasks, cycle and stale-version rejection, link retirement, threaded
comment edits/tombstones, pinned reads, receipt replay after reconstruction,
checkpointing, recovery and manifest-last copy. Fake storage and real RustFS
qualification are recorded separately. Transport gates additionally require home
authorization, target-code authorization, executed HTTP contract capture, CLI
publication over local RPC and unknown-outcome status. Browser gates cover
project/task selection, state and hierarchy, comment interactions, automatic
refresh, accessibility and a cold startup. Visual baselines remain pending.

E-CI-COLD-RESPONSE-001 retains the JST-WORK-008 cold-browser acceptance while
making receipt observation navigation-safe: the harness consumes the committed
WorkStore response body immediately, before the accepted mutation can navigate
or refresh the page. Request identity, committed status, resulting view and
product behavior remain unchanged.
