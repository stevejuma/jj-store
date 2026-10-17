# Embedding conversation workflows

`jj-store-conversations` contains reusable headless conversation mechanics.
Its initial boundary is the Control V3 event stream: private prefix buffering,
final host scanning and bounded per-participant assistant/tool accumulation.
The library imports no JJ service, runtime, object store or transport.

Implement `privacy::Sanitizer` using an admitted native privacy policy. The
capability must perform prefix-safe replacement and scan complete output frames,
including keys and nested tool content. Return fixed safe errors when privacy is
unavailable. jj-store adapts its existing bounded `TranscriptSanitizer`; its
policy IDs, scanner versions and sanitized payload limits remain unchanged.

A host keeps one raw event owner. Pass raw frames to `StreamPrivacy` for one
participation or `control_v3::Stream` for several isolated participations. All
emitted frames pass the injected scanner. Apply `Stream::observe` to the safe
frames to accumulate assistant text and merge tool snapshots. Preserve frame
ordering when a boundary emits a buffered prefix before a terminal frame.

Durably journal sanitized frames and their exact progress before displaying them
or acknowledging the runtime. The stream library supplies neither a durable
receipt nor an acknowledgement owner. A renderer subscription must not consume
raw runtime events. Undecidable prefix buffers stay memory-only and have no
serialization or debug representation. Close and disable must drain the owning
pipeline before releasing authority.

Message publication, prompt delivery and captured answers remain separate
outcomes. Persisted participant attribution does not resolve a device-local
executable or restore a provider session. Repository stores remain selected
through the [injectable store interfaces](architecture.md). The remaining
M33 conversation ports, typed client and Notes integration are tracked by the
staged M33 specification.
