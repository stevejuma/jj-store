# Rich history query delivery

Operation-pinned history reads MUST expose the pinned JJ 0.45.1 callable catalog
and standard aliases independently of device or server user configuration.
Commit revsets use explicit workspace, author email and frozen time context;
`mine()` requires email and `@` requires a workspace. Native content and
cross-operation predicates retain their existing admission and publication
restrictions.

Operation metadata queries MUST filter the full bounded selected ancestry before
pagination, bind exact query text into continuations, charge work for every
scanned operation and retain actual parent IDs and native ordering. Spaces and
explicit AND combine `description:`, `user:`, `host:`, `workspace:`, `id:`,
`after:`, `before:`, `snapshot:` and one `from:` selector; bare text matches
descriptions. Quoted text supports escaped quotes and backslashes. Text matches
case-insensitive substrings, IDs match hexadecimal prefixes, dates use UTC
`YYYY-MM-DD` against end time with inclusive after and exclusive before, and
snapshot accepts true/false. `from:` accepts JJ operation selectors only within
the displayed snapshot's ancestry. OR, NOT, commit revsets, missing values and
malformed selectors fail explicitly. Queries are bounded to 4096 bytes and 64
terms. Diagnostics are bounded and expose no storage or credential details;
these reads never reconcile heads or publish state.

LocalTrusted query metadata MAY supply a bounded default author email from host
JJ configuration, including JJ_EMAIL. Registered local folders fall back to
repository JJ then Git configuration when the host has no usable email.
Configuration reads MUST remain read-only, expose only the email and never load
aliases or other configuration into query evaluation. Authenticated native
serving MUST omit host identity hints; explicit request context remains the
query authority. Secure repository fallback requires a metadata binding that
resolves to the exact registered metadata directory.
