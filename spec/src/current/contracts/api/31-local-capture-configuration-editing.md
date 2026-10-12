# Local capture configuration editing

JST-API-001 includes installation-scoped `GET /v1/local/capture/settings` and
`POST /v1/local/capture/settings`. Only current LocalTrusted Admin authority may
read roots or save configuration. Bodies are strict, actor-free version 1 and
carry the exact saved-file revision plus typed capture configuration. A save
replaces only `agent_capture` under the shared settings-file lock; it preserves
unrelated TOML values and detects stale file revisions, including concurrent
search editing. Bounded same-directory atomic replacement and file/parent
synchronization precede acknowledgement. A failed acknowledgement is an unknown
outcome: clients retain their draft and compare a fresh saved configuration
before retrying. Loaded-effective settings and activation-required state are
distinct from saved configuration and runtime health. The API never executes an
agent or exposes a generic filesystem/config editor.

Configuration observations and saves MUST remain available while transcript
inspection occupies the runtime control gate. They use bounded file reads and
the shared settings-file lock independently from capture journals, spools or
worker command ownership.

Capture configuration accepts the two Codex profiles, separate Cursor Desktop
and CLI transcript profiles, and OpenCode read-only API (at most five selected
profiles). Harness settings carry an optional endpoint only for OpenCode; that
profile requires an explicit credential-free literal-loopback root URL and no
filesystem roots. Cursor profiles expose resolved roots and independently
qualified product versions. Existing saved/effective revision, conditional-save
and explicit activation semantics remain unchanged. Export is not editable or
enabled.

Grouping harness controls into shared presentation rows MUST preserve each
profile's independent enablement, roots or endpoint in the saved draft. Opening
or closing a harness row performs no configuration save or capture command.

Configured runtime coverage supports up to five independently qualified product
profiles. Cursor sources use cached ordinary coverage and repository binding,
but never advertise an inactive-history import target. OpenCode endpoint errors
do not turn the entire capture service unavailable. Source GETs remain cached
and perform no live directory scan or endpoint request.

Saved capture configuration may be activated through the shared runtime reload.
The capture owner drains before replacement; source generations, policy, spools
and original import receipts remain retained. Busy or failed drain cannot start
a second capture writer. Loaded settings remain effective until replacement is
ready, and hidden client drafts are never reset by reload observations.
