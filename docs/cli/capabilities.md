Report native backend capabilities and their current verification scope.

`jj store capabilities --format json` returns a versioned command matrix. Native
commit, operation and evolution histories are separate. Git transport commands
require a Git backend and are unsupported on native repositories. Destructive GC
remains disabled. A library test is not executable parity evidence: commands
without direct acceptance are reported as unqualified.
