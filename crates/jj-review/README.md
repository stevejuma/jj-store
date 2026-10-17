# jj-review

Repository-owned review anchors, comments, thread state and immutable review
objects. Anchors identify exact code or agent artifacts, with portable source
evidence where the selected repository profile supports it.

Construct and validate an `ArtifactAnchorV1` and a `ReviewCommandV1`, then
submit them through a ReviewStore provider. The exported codecs and resolver
contracts support canonical records and source validation; providers own
persistence, authorization and receipt replay.

See the [review guide](../../docs/site/guides/reviews.md) and
[publication model](../../docs/site/technical/publication.md).
