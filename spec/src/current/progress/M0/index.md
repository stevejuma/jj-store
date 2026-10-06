# M0 tasks and gates

| ID   | Status | Depends   | Requirements                     | Gate                                                        | Next                                  | Evidence          |
| ---- | ------ | --------- | -------------------------------- | ----------------------------------------------------------- | ------------------------------------- | ----------------- |
| M0   | Done   | Bootstrap | JST-COMPAT-001,JST-COMPAT-002    | Compiled extension and exact RustFS contract qualified      | Maintain pinned compatibility probes  | E-M0-001,E-M0-002 |
| M0.1 | Done   | Bootstrap | JST-COMPAT-001                   | Pin references/licenses and complete trait/codec matrix     | Re-audit on upstream upgrades         | E-M0-001          |
| M0.2 | Done   | M0.1      | JST-JJ-001,JST-JJ-002,JST-ID-002 | Compile custom CLI/RPC context and query spikes; golden IDs | Retain probes across upstream changes | E-M0-002          |
| M0.3 | Done   | M0.1      | JST-OBJ-002                      | Real image create/CAS/range/restart/fault qualification     | Requalify topology or image changes   | E-M0-001          |
<!-- generated-navigation -->
## Pages

- [history](history/index.md)
