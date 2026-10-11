# Settings configuration coverage

| Format-2 fields                                                                                                                                        | Settings location                              | Editing policy                                                                |
| ------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------- | ----------------------------------------------------------------------------- |
| `limits.*`                                                                                                                                             | Local service / Limits                         | Conditional save; next startup                                                |
| `protection.bookmarks`, `protection.commits`                                                                                                           | Local service / Protection                     | Installation-wide; next startup                                               |
| `replication[].repository`, `upstream`, `insecure_loopback`, `token_environment`, `ca_certificate`, `limits.*`                                         | Sync / Configuration                           | Selected native repository; credential name only; next startup                |
| `search[].enabled`, `index_bytes`, `source.index_source`, `source.scopes`, `source.include_prefixes`, `source.exclude_prefixes`                        | Search / Configuration and File scopes         | Conditional save; explicit Apply                                              |
| `search[].poll_millis`, `timeout_seconds`, `snapshot_every`, `source.max_text_bytes`, `source.max_files`, `source.max_work`, `source.max_output_bytes` | Search / Advanced                              | Conditional save; explicit Apply                                              |
| `search[].repository`, `token_environment`                                                                                                             | Repository settings URL / local access profile | Repository-bound; LocalTrusted forbids worker token references                |
| `agent_capture.*`                                                                                                                                      | Agents / Configuration                         | Existing qualified editor; next startup                                       |
| `format`, `state_dir`, `operator_policy`, `access.*`                                                                                                   | Local service / Launch                         | Read-only setup identity/access; no migration                                 |
| `bind`, `public_url`, `insecure_loopback`, `tls.*`, `web_root`, `development_browser_annotations`                                                      | Local service / Launch                         | Read-only managed launch/lifecycle fields                                     |
| `storage.*`                                                                                                                                            | Local service / Storage                        | Read-only authority location and credential references; migration unqualified |
| `backup[]`                                                                                                                                             | Local service / Storage                        | Read-only; execution deferred                                                 |

Credential environment/OS names and private-file paths in this inventory are
references only. The Settings API neither reads their credential contents nor
accepts literal secret values. Search source field omissions use established
extraction defaults; current clients submit the complete typed configuration.

The configured local capture extension adds independently qualified Cursor
Desktop 3.22.7 and CLI 2026.09.26-dd393fe, with an explicit transcript mode in
owned Cursor hook setup. That mode provides current binding/lifecycle without
replaying hook messages alongside transcript messages. OpenCode 1.18.30 uses the
configured read-only loopback API. All settings are editable under Agents and
apply on the next backend startup; capture never starts a harness.

Cursor transcript setup is explicit: enable Cursor in the device registry, then
run `jj agent setup cursor --transcript-capture` in the linked native workspace.
CLI version 2026.09.26-dd393fe and Desktop 3.22.7 are separate product profiles.
`--dry-run` previews the exact owned handlers; removal preserves unrelated
configuration and restores the original bytes. Changing an existing owned setup
mode requires exact removal followed by setup; drift is never overwritten.
Transcript hooks write only private binding/activity hints and never publish
hook messages alongside JSONL messages. Nullable paths await a later hook.
Desktop defaults to `~/.cursor/projects`; CLI uses `CURSOR_DATA_DIR/projects`
when configured, otherwise the same home convention. Custom CLI roots must agree
with the harness data-directory configuration and server-approved roots.

The composed LocalTrusted browser installation can create a native repository
through its installation-owned manager without invoking a CLI subprocess or
creating a physical workspace. CLI provisioning contracts remain unchanged;
filesystem-only installations do not advertise native creation.
