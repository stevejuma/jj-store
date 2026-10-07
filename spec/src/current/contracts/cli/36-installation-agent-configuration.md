# Installation agent configuration

Installation agent settings extend the JST-CLI-001 service profile. Saved
defaults, environment references and supervised compute have independent
lifecycle/revision boundaries outside checkouts. Every launch/catalog resolves
the same effective configuration. Secrets never enter process arguments or
diagnostics. Explicit retained settings writes preserve original request
identities; worker configuration changes take effect on the next start. Existing
native identity, authorization, receipts, manifest publication and
no-prompt-replay contracts remain unchanged.
