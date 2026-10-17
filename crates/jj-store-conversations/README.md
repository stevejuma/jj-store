# jj-store-conversations

Headless conversation coordination shared by JJ applications. The library
currently provides Control V3 stream privacy and bounded assistant/tool
projections, with a host-injected sanitization capability. It has no server,
transport, Tauri, runtime or repository storage dependency.

Implement `privacy::Sanitizer` with the host's admitted privacy policy, then pass
raw frames through `privacy::StreamPrivacy` or the per-participant
`control_v3::Stream`. Apply `Stream::observe` to safe frames for complete
assistant text and tool snapshots. Keep private prefixes in memory; durably
capture safe output before display and provider acknowledgement.

The host owns credentials, source admission, journal lifetime, publication and
UI delivery. A participant ID is attribution and grants no execution authority.
See the [conversation embedding guide](../../docs/site/technical/conversations.md) for the
pipeline and ownership contracts.
