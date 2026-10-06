# App-managed operational configuration

JST-CLI-005 format-2 operational settings may be edited through the current
LocalTrusted Admin Settings API: service limits, installation-wide protection
and repository replication configuration, plus the existing Search and Agents
editors. Shared-file conditional writes preserve unrelated values. Operational
saves apply on next startup without storage migration, credential provisioning,
peer contact, repository publication or implicit restart. Identity/access,
listener/TLS, bundle paths and authority storage remain launch-managed read-only
inventory; backup execution remains deferred. Search's advanced polling, timeout
and snapshot interval participate in its explicit Apply lifecycle.
