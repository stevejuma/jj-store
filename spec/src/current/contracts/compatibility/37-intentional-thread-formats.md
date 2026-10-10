# Intentional Thread format compatibility

JST-COMPAT-001 retains JJ 0.45.1 and the independent native/local identity
profiles. Optional Thread purpose and Codebase fields are omitted when absent,
preserving legacy canonical version-1 bytes. Generated HTTP contracts add
explicit installation-local Thread configuration and an optional exact send
witness; they do not remap existing object, operation, repository or agent
identities. Focused model and generated contract checks qualify this extension
separately from platform, storage and installed-provider acceptance.
