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
boundaries; private media never requires credentials in an image URL.

Startup audits reachable selected directory records and media. Transport
unavailability retains the existing server startup behavior; directory reads and
mutations remain unavailable until the store can be reached. Missing selected
objects or invalid canonical records are corruption and fail the startup audit.
No local fallback is authorized by an outage.
