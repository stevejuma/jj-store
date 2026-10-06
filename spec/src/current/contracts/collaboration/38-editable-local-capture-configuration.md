# Editable local capture configuration

JST-COLLAB-017 now includes a LocalTrusted Admin configuration editor. The
installation owns saved capture enablement, qualified Codex CLI/Desktop
profiles, approved directory overrides and bounded reader, discovery, source and
spool limits. Empty directory overrides use the server process's resolved
harness defaults. The editor MUST show saved versus startup-effective settings
and resolved default roots even when capture is disabled. Saving MUST NOT
initialize capture journals, renew import consent, repair state or change a
running worker. Changes take effect on the next server startup; pending
configuration MUST withhold runtime mutations until that restart. Existing
policy, source identities, prepared requests and receipts remain retained. Other
harness startup editing and arbitrary server TOML/credential editing remain
deferred.
