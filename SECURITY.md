# Security policy

Report vulnerabilities privately:
https://github.com/stevejuma/jj-store/security/advisories/new. Do not put
credentials, private repository data or raw transcripts in issues.

The 0.1 series is experimental; security maintenance targets the latest release.
Use explicit versions/digests and verify checksums and provenance. Shared
serving requires authenticated TLS, private installation state and current
authorization.

Release jobs use scoped permissions, reviewed Action pins and OIDC where
supported. Fork PRs receive no publishing credentials. Container releases
include an SBOM and signed provenance. High/critical findings require
remediation or an explicit reviewed exception with an expiry.
