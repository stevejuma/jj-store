# Troubleshooting

Start with the current context and the original request identity. Recreating
state often removes the evidence needed to resolve an interrupted operation.

| Symptom                                    | Check first                                                          | Next action                                                                      |
| ------------------------------------------ | -------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| Repository is absent                       | Installation catalog, selected token, device folder registration     | Verify discovery and current repository permission separately                    |
| Command result is unknown                  | Original request ID and prepared intent                              | Inspect status or retry the identical request                                    |
| New content is not searchable              | Search configuration, generation and lag                             | Wait for the selected committed position or repair/rebuild the derived index     |
| Code view did not change                   | Explicit historical operation selection                              | Return deliberately to a current view                                            |
| Capture has no new session                 | Exact harness version, roots, armed discovery and repository binding | Start a supported new turn or explicitly import history                          |
| Configuration saved but behavior unchanged | Saved versus effective settings                                      | Apply/restart the affected worker as indicated                                   |
| Local folder action is unavailable         | Repository profile and capability                                    | Use the supported foreground JJ workflow rather than substituting native actions |
| Integrity says incomplete                  | Prepared target and resource/time budget                             | Retain the report; do not label the repository healthy                           |
| Help fails to load                         | Paired browser/docs build and asset manifest                         | Rebuild/stage the complete bundle and retry                                      |

## Service startup

`jj-store config check` validates the file without contacting storage. Startup
adds runtime checks such as installation ownership, storage locking and the
sentinel. A configuration pass does not prove those runtime checks will pass.
Use the exact generated state directory and paired executables. Do not run a
second server against a locked namespace or initialize over existing state.

## Credentials

`JJ_STORE_TOKEN` overrides OS-held client credentials. Logging out removes the
selected stored account, not an environment variable in the calling process. An
expired or revoked grant can make a previously valid bookmark or cursor
unreadable. The same token may still have independent access elsewhere.

## Capture and UI feedback

A cold sanitizer can be busy for minutes in a debug build. Inspect the original
import receipt rather than issuing multiple replacements. Loading is not
recovery, and dismissed notices need not reappear on every refresh. Field and
unknown-operation status remains near its control so a draft can be recovered.

When reporting a problem, include the application build, repository profile,
operation/request IDs, command and nonsecret error category. Do not include
bearer tokens, private credentials or raw agent transcripts by default. Preserve
state before experimenting with reset; ordinary development restart preserves
fixtures and does not require reseeding.
