# App-managed operational configuration

JST-CLI-005 format-2 operational settings may be edited through the current
LocalTrusted Admin Settings API: service limits, installation-wide protection
and repository replication configuration, plus the existing Search and Agents
editors. Shared-file conditional writes preserve unrelated values. Operational
saves remain inactive until explicit reload or the next startup. Saving performs
no storage migration, credential provisioning, peer contact, repository
publication or implicit restart. Identity/access, listener/TLS, bundle paths and
authority storage remain launch-managed read-only inventory; backup execution
remains deferred. Search's advanced polling, timeout and snapshot interval
participate in its explicit Apply lifecycle.

Operational saves remain distinct from activation. The application reload API
activates supported saved domains without replacing the server process. Runtime
configuration may replace drained workers while retaining their durable state.
Installation/storage/access/listener and bundle identities remain fixed; backup
execution remains deferred. Reload does not install binaries or change
credentials. Agent/compute processes retain active work; pending launch changes
are reported separately from server configuration activation.

Workflow admission and optional local host settings are operational domains. The
native CLI composition supplies the optional event/action runtime; the core
server owns configuration and admission, not executable dispatch. Reload keeps
the server process and listeners. The standalone host JSON format remains valid;
managed changes apply between settled polls/jobs, never by replaying an attempt.
