# WorkStore service and transports

A project link records only an opaque RepositoryId and grants no permission.
Metadata reads/writes require current home access; opening linked code follows
the existing independently authenticated target repository route. Receipt replay
and every resumed WorkStore page recheck current authorization.

Git transport permits only device-approved executable paths, supported OS
credential helpers and explicit CA files. Its supervisor entry point does not
load native credentials; those credentials are also removed from its inherited
environment. Authenticated HTTPS qualification uses generated fixture-only
credentials and trust, including refusal without the approved CA. Neither
bindings nor durable attempts contain the credential secret.
