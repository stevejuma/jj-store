---
schema: 1
id: E-LOCAL-PROFILE-MEDIA-001
---

# E-LOCAL-PROFILE-MEDIA-001

<!-- evidence-field:id markdown -->
## Id

E-LOCAL-PROFILE-MEDIA-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-23
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M26
- M26.1
- M26.2
- M26.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-010
- JST-OBJ-006
- JST-COLLAB-012
- JST-CLI-001
- JST-CLI-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/normalize_image.rs
- crates/jj-store-server/src/directory/images.rs
- spec/src/current/contracts/security/11-installation-profiles-and-emoji.md
- spec/src/current/contracts/collaboration/16-persistent-threads-and-reactions-m15.md
- spec/src/current/contracts/cli/02-cli-and-server-commands-continued-2.md
- spec/src/current/progress/M26/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/src/normalize_image.rs#large_jpeg_avatars_resize_and_keep_aspect_ratio
- crates/jj-collab/src/normalize_image.rs#resized_jpeg_avatars_apply_exif_orientation
- crates/jj-collab/src/normalize_image.rs#oversized_emoji_rasters_normalize_and_keep_transparency
- crates/jj-collab/src/normalize_image.rs#source_file_over_two_mib_is_normalized_and_small_sources_keep_bytes
- crates/jj-collab/src/normalize_image.rs#avatar_decode_and_source_limits_reject_before_publication
- crates/jj-store-server/src/directory/images.rs#large_jpeg_avatars_resize_and_keep_aspect_ratio
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-server directory::images::tests --lib --locked
```

**Notes**

Before and after extraction: 5 pass each. After run emits the existing large __eh_frame linker warning; no test failures.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-collab normalize_image::tests --lib --locked
```

**Notes**

5 shared normalizer tests pass.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-collab --locked
```

**Notes**

58 tests pass, 0 fail; doc tests have 0 cases. Direct crate suite covers both the original ingest contract and the new shared preparation entry point.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Desktop native persistence and cold app acceptance are recorded by jj-desktop
  independently.
- Broader server, release, platform and visual gates remain outside this
  extraction.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.99; jj-collab 0.1.0; jj-store-server 0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing ImageAssetV1 and raster bytes; no canonical or request/receipt format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Extract the existing server normalizer into jj-collab for direct native Profile/Emoji consumers. The server preserves admission, receipts, error mapping and the original five regressions.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Consume the public helper in JJ Desktop installation-owned settings and record native acceptance there.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-API-010/JST-OBJ-006 preparation now has a public
  transport-independent helper. JST-COLLAB-012 retained limits and canonical
  bytes, and JST-CLI-001/002 command/HTTP contracts are unchanged; mapped
  chapters document the boundary.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M26 and its selected subgates remain Partial. Shared preparation
  verification does not qualify Desktop, broader server, or visual release
  gates.

```
<!-- /evidence-field -->

