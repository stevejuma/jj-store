# jj-store-conversations

Headless conversation coordination shared by JJ applications. The library
provides durable prompt coordination, Control V3 stream privacy and bounded
assistant/tool projections, with host-injected capabilities. Default builds have
no server, transport, Tauri, runtime or repository storage dependency.

Implement `privacy::Sanitizer` with the host's admitted privacy policy, then
pass raw frames through `privacy::StreamPrivacy` or the per-participant
`control_v3::Stream`. Apply `Stream::observe` to safe frames for complete
assistant text and tool snapshots. Keep private prefixes in memory; durably
capture safe output before display and provider acknowledgement.

The host owns credentials, source admission, journal lifetime, publication and
UI delivery. A participant ID is attribution and grants no execution authority.
See the
[conversation embedding guide](../../docs/site/technical/conversations.md) for
the pipeline and ownership contracts.

Implement `coordination::Types` with your existing command/outcome types and
inject `Admission`, `Context`, `Journal` and `Runtime` ports into `Coordinator`.
`send` prepares every recipient before dispatch and retains uncertain delivery
without replay. Ports may borrow a scoped host lease. Store publication,
workspace preparation and exact journal receipt storage remain host-owned.

For native event capture, construct `capture::NativeCapture` with an opaque host
binding, policy ID, admitted sanitizer and `DurableCapture` sink. Pass each
native processing ordinal and facts/event cursor to `capture`. Safe output
returns only after the sink confirms the exact batch identity. Deliver every
returned frame in order: a boundary may flush a private prefix before its
terminal frame. The host owns durable storage, receipt recovery and callback
drain; errors never select another writer or unprocessed output.

Enable the optional `native-privacy` feature to use the qualified Agent
Redaction transcript policy with `NativeCapture::with_native_privacy`. Supply an
`Arc<native_privacy::TranscriptSanitizer>` whose roots, cache and exact secrets
have already been admitted by the host, plus your durable sink. The constructor
selects the existing transcript policy ID. Default builds retain the injected
privacy port without linking the scanner. Full integration and admission details
remain in the conversation embedding guide.

Enable `store-capture` to use `store_capture::StoreCapture` as the native
capture sink. Supply explicitly selected, repository-bound Agent/Thread stores,
`ContextV1` and an implementation of the private durable `Journal` port.
Candidate and exact provider preparation are retained before publication;
`recover(ordinal)` checks the original receipt without executing again. The
feature adds only owned domain/core interfaces. Hosts still own backend
selection, the journal's exclusive lease and fsync, admission and callback
drain.
