# Installation agent configuration

Installation agent settings extend the JST-CLI-001 service profile. Saved
defaults, environment references and supervised compute have independent
lifecycle/revision boundaries outside checkouts. Every launch/catalog resolves
the same effective configuration. Secrets never enter process arguments or
diagnostics. Explicit retained settings writes preserve original request
identities; worker configuration changes take effect on the next start. Existing
native identity, authorization, receipts, manifest publication and
no-prompt-replay contracts remain unchanged.

ChatGPT plan registrations extend the existing application-owned LocalTrusted
configuration. The local JJ Agent receives renewable access tokens through the
account-bound private broker; OAuth and Keychain operations remain outside
jj-acp and repository state. This provider adds no CLI authentication command or
Codex CLI dependency. Existing explicit start/restart and request-recovery
contracts remain in force; the browser's explicit authorization control owns
registration and account selection.
