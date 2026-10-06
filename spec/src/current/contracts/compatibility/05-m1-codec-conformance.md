# M1 codec conformance

The frozen M1 profile vectors cover all seven stored kinds. Native codec tests
compare supported kinds against the pinned SimpleBackend and SimpleOpStore; copy
and submodule coverage uses the audited native ContentHash profile because
SimpleBackend cannot persist those variants. Codec conformance does not qualify
the later storage transport or publication coordinator.
