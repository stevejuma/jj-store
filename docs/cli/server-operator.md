# Operator credentials

The local operator policy authorizes repository creation. It never grants access
to repository contents. `operator init` creates a policy; an identical retry is
safe. `operator grant` adds an explicitly selected credential. `operator list`
shows public grant metadata, and `operator revoke` removes one token ID.

Provision with a canonical `--endpoint`, an explicit `--principal` identity, and
`--policy PATH` outside checkouts. `JJ_STORE_OPERATOR_TOKEN` supplies an
existing automation credential. Otherwise provisioning reuses or creates a token
in the OS credential facility for that endpoint. Secrets never appear in output.
Policy files retain only verifiers and are reloaded for each creation request.

Repository credentials remain separate: `JJ_STORE_TOKEN` or `jj store login`. No
plaintext credential fallback is available.

Use `jj-store token` for repository grants and revocations. Those requests
require a current repository admin credential; the creation operator cannot
authorize them.

Use `jj-store token generate` once when one manual credential should access
several repositories. Grant it separately through each repository policy; the
installation registry uses its public token ID only to find candidates and each
repository reauthorizes the complete credential.
