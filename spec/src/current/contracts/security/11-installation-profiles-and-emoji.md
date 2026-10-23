# Installation profiles and emoji (M26)

| ID          | Requirement                                                                                                                                                                                                                                                                                                                                                                            |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-SEC-008 | Installation profile and emoji mutations MUST authenticate a server-owned token binding or the admitted LocalTrusted administrator; repository-selected principal IDs MUST NOT authorize personal metadata edits. Private email and token verifiers MUST never appear in public projections. Identity provisioning MUST require the independent manage-identities operator capability. |
| JST-API-010 | Installation directory APIs MUST expose revision-conditional profile/emoji edits, bounded profile/library reads, validated media uploads and actor-bound durable command status with stable request identities.                                                                                                                                                                        |
| JST-UI-021  | Settings MUST expose owned Community-derived Profile and Custom emoji pages with retained drafts, accessible controls, contextual validation, application notifications and automatic freshness.                                                                                                                                                                                       |
| JST-OBJ-006 | Installation metadata and raster media MUST use immutable SHA-256-addressed objects selected by one installation manifest CAS, bounded catalogs and durable receipts; unknown writes MUST remain queryable without changing request identity.                                                                                                                                          |

The installation identity selects an independent directory namespace in the
configured authoritative object store. Repository manifests and token policies
do not select this installation state. Reads never initialize it. The first
explicit mutation creates it. Filesystem-only installations advertise the
feature as unavailable; no local JSON fallback or implicit storage migration
exists. Serving remains single-writer and destructive cleanup remains disabled.

Profiles retain display name, Markdown description, optional owner-only email
and an optional uploaded image, HTTPS URL or bounded Community emoji avatar.
Uploaded bytes and static previews are retained separately. Public responses
derive Gravatar from normalized email only when no explicit avatar is selected;
they never disclose email. Broken images fall back to initials. Profiles do not
authorize repository access or remap historical actors.

Authenticated users enroll existing bearer tokens through trusted operator
administration. Bindings retain installation-bound verifiers, principal and
optional expiry, with explicit revocation. Repository creation permission does
not imply identity management. Personal settings remain available to an enrolled
user with no repository grant; anonymous directory access remains forbidden.

Custom emoji are owner-managed and installation-wide, with unique active
shortcodes. Raster uploads use the existing bounded PNG/JPEG/GIF/WebP inspector.
Replacement and retirement retain prior immutable definitions and image bytes.
An upload publishes owner-accessible draft media; selecting it in a profile or
emoji definition is a separate explicit save.

Avatar and custom emoji uploads share a binary transport at
`POST /v1/directory/image-uploads/{request_id}`. Admission checks the personal
identity before consuming at most 20 MiB of PNG/JPEG/GIF/WebP source bytes.
Decoded input is bounded to 8192 pixels per side and 64 MiB of RGBA pixels. The
server preserves exact bytes for sources already within retained-media limits;
larger inputs are resized to fit 512 × 512, retaining aspect ratio and EXIF
orientation, and encoded as a static PNG. Resized animations retain their first
frame. Source files are not persisted. Retained assets and previews still meet
the existing 2 MiB, 512 × 512, 100-frame and 64 MiB domain limits. The upload
returns the existing owner-only media receipt; profile and emoji Save remain
explicit. Receipts bind the original source digest and request identity, so
retries recover the original result without re-normalizing. Legacy upload
commands and legacy emoji receipts retain their original intent encoding.

The transport-independent `jj-collab::normalize_settings_image` helper owns
these same source/retained raster bounds for native consumers; the server
delegates to it. This reusable preparation function grants no installation,
repository or publication authority. Native applications retain their own
installation storage, admission, request receipts and explicit profile/emoji
saves.

The custom emoji chooser keeps source files and previews in browser session
memory until Save emoji. That explicit save first obtains normalized owner-only
media, then selects it in the retained definition request. Unknown outcomes keep
the original upload and definition intent; choosing or abandoning a file cannot
publish installation media. The profile photo picker retains its explicit draft
upload behavior.

Profile and emoji forms use the shared modern Settings cards, icon headings,
divided labelled rows and bottom action footers. Profile info includes the
copyable principal identity; it has no separate identity disclosure section.

Repository use retains the selected definition and complete image closure with
the authored message/reaction. It must not depend on a future directory read.
Repository-local libraries keep their existing meanings; source-library
identities disambiguate collisions. Previously encoded records keep their bytes.
Retirement prevents new preparations; prepared commands retain their original
snapshot and exact receipt identity. No capture or runner dispatch is implied.

Community supplies presentation source only. All copied source, helpers and CSS
live in web/src/lib with pinned attribution in docs/ui-provenance.json. Released
Design Core and Mira remain foundational dependencies. No Nostr persistence,
Community checkout imports, email-based identity inference or agent journal
migration is introduced by this feature.

Browser image policy admits in-memory blob URLs for authenticated directory and
repository media. Script, frame and connection sources retain their existing
boundaries; private media never requires credentials in an image URL. The Vite
development image policy admits the same blob previews; its existing script,
worker, origin, private-file and API proxy restrictions remain intact.

Startup audits reachable selected directory records and media. Transport
unavailability retains the existing server startup behavior; directory reads and
mutations remain unavailable until the store can be reached. Missing selected
objects or invalid canonical records are corruption and fail the startup audit.
No local fallback is authorized by an outage.
