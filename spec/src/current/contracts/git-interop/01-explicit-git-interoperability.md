# Explicit Git interoperability

E-CI-BROWSER-001 retains the original real-RustFS external Git merge, private
cache loss, cold restart and browser-refresh acceptance parent in an independent
CI job. Its child requires that parent's exact workspace and cannot run as an
unprovisioned generic browser test. Fixture-root selection changes only where
current test tools/assets are read; transport, bookmark profiles, leases,
identity mappings and native publication remain unchanged.

Native jj-store remains the working repository. Git fetch and push are explicit
CLI operations over a private disposable bare cache. The attached bridge
proposal is reference material; these requirements own the accepted contract.
The bridge follows GB0 through GB6 as M12.1 through M12.7 in the single ledger.
Its native WorkStore prerequisites are the original qualified model, command and
repository browser gates M11.1-M11.3. Later installation Projects hub and
shared-package UI qualification in M11.4 does not change those native bridge
gates or their recorded acceptance.
