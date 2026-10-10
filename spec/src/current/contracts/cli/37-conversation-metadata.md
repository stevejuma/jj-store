# Conversation metadata administration

Conversation metadata uses internal collaboration owners where no intentional
Thread is selected. These owners do not enter ordinary Thread directories or
become primary session assignments. Existing command and receipt semantics
continue to resolve exact authorized owners.

JST-CLI-004 provides jj-store conversation-metadata with one explicit --endpoint
http://127.0.0.1:PORT/ (or literal IPv6 loopback). Remote hosts, credentials in
URLs, redirects and implicit service selection are refused.

- read --session ID [--repository REPO] reads current authorized details.
- command --input FILE submits one bounded version-1 JSON command.
- job --request ID inspects the original retained request.

The input file is capped at 32 KiB before allocation can grow past the limit.
Responses are read incrementally with a one-MiB cap. Output is typed JSON;
non-success responses preserve the original intent. The CLI does not silently
retry an unknown command or create an instruction-only conversation turn.
Commands share the HTTP/native/local authorization, immutable metadata and
receipt policy in the conversation metadata contracts.
