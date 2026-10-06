# M2 tasks and gates

| ID   | Status | Depends | Requirements                        | Gate                                          | Next                                             | Evidence          |
| ---- | ------ | ------- | ----------------------------------- | --------------------------------------------- | ------------------------------------------------ | ----------------- |
| M2   | Done   | M1      | JST-PUB-001,JST-PUB-006             | Publication fault matrix and cold replay pass | Retain publication and cold recovery conformance | E-M2-001,E-M2-002 |
| M2.1 | Done   | M1      | JST-PUB-001,JST-PUB-002             | Complete inventory and unique candidate CAS   | Retain publication and cold recovery conformance | E-M2-001,E-M2-002 |
| M2.2 | Done   | M2.1    | JST-PUB-003,JST-PUB-004,JST-PUB-005 | Retry receipts and unknown outcome recovery   | Retain publication and cold recovery conformance | E-M2-002          |
| M2.3 | Done   | M2.2    | JST-PUB-006                         | Checkpoint equivalence and cache-free restart | Retain publication and cold recovery conformance | E-M2-002          |

<!-- generated-navigation -->

## Pages

- [history](history/index.md)
