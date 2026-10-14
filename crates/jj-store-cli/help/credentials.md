Manage repository credentials outside the checkout.

```sh
jj store login --endpoint https://store.example --repo-id REPOSITORY_ID
jj store logout --endpoint https://store.example --repo-id REPOSITORY_ID
```

Login prompts without echo; `--token-stdin` reads one token from stdin for
provisioning. Login verifies access before saving to the OS credential facility.
Credential accounts bind the canonical server URL and repository ID. No
plaintext fallback is provided. Logout removes only that exact account. An
injected `JJ_STORE_TOKEN` overrides the OS facility; logout reports that the
environment must be cleared separately. Never pass a token on the command line.
