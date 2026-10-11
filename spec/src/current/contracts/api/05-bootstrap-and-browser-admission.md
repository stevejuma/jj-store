# Bootstrap and browser admission

`GET /v1/bootstrap` precedes repository UI and returns access mode, public
capabilities and authoritative-storage readiness. Repository discovery is a
separate bounded `GET /v1/repositories` request. LocalTrusted returns the
identity-bound installation catalog; authenticated mode accepts one Bearer
credential, narrows by its public token ID and reauthorizes every candidate
against current repository authority. Reserved backup fields remain unavailable
while backup and restore are deferred. Neither response contains an internal
credential, and discovery metadata never confers repository access.

Every `/v1` request supplies `X-JJ-Store-Protocol: 1`. The server validates the
configured literal loopback authority, rejects forwarded-host trust, null or
cross-origin browser requests, cross-site Fetch Metadata and method overrides,
and accepts only endpoint-specific JSON or binary request media types. SSE uses
the same authority, origin, protocol and access checks. LocalTrusted rejects a
Bearer header and binds the installation HTTP principal. Authenticated discovery
and repository requests require the same manual bearer credential; browser
credentials remain memory-only. OpenAPI declares deployment-dependent access
rather than promising an unauthenticated public listener.

Review thread summaries include optional root-pinned presentation with creation
actor/time, last accepted event time and a nullable current-comment excerpt.
Repository directory presentation composes existing snapshot, log, WorkStore
browse and ReviewStore page contracts with separate authorization and bounded
pages; it never adds persisted discovery ownership or reconciles operation
heads.

The transient Settings presentation composes only existing bootstrap, repository
discovery, capabilities, snapshot, search-status and index-task contracts. The
repository Details Settings action selects the transient Repository section over
the already connected repository. JST-API-005/008 remain unchanged: the action
adds no repository-open request, endpoint, authorization or persisted route and
retains the exact selected snapshot and code context. The production controller
retains each discovered repository's effective permission and exposes the
already captured capabilities and snapshot authority to presentation. Search
status is requested only when search is advertised, and its failure does not
replace repository content or connection state. Index preparation submits the
operation reported by an index-lag response when present, otherwise the
explicitly selected snapshot operation; concurrent unselected heads cannot start
a task. Repository switching, disconnection and revocation close the source
session, abort outstanding requests and clear this settings projection. This UI
adds no server route, schema or generated contract.

The sidebar Add project action uses the existing permission-gated WorkStore
create dialog and home selector. Sidebar repository-add actions reuse the same
LocalTrusted folder manager as Projects and preserve explicit Inspect/Add and
pending registration identities. Activity row navigation uses the existing hash
route with the row's repository, operation and complete commit/change IDs.
Parent-task and picker indicators use already loaded metadata. These changes add
no transport or persistence contract.

Projects header Add-menu dispatch and repository dialogs reuse the existing
local-folder registration and WorkStore set_repository contracts. The local
registration manager remains alive outside the transient menu. Moving link
selection into a dialog retains picker/manual full-ID selection, permission and
version gates, pending request identity and receipt recovery; accepted metadata
refresh closes a successfully linked dialog. Settings-style form composition
changes presentation only and adds no request or persistence format.

The transient destination consumes the shell's existing compact state so a
collapsed desktop rail exposes Back and each settings section as labelled icon
controls while keeping section selection and repository authority unchanged.
Settings page titles and section icons precede subsection tabs in the content
navigation; capability-gated refresh actions sit at its far right as labelled
icon buttons with tooltips. Back remains in the sidebar. Settings content has no
extra top padding, and action groups stack with equal-width buttons except on
Appearance. These layout choices preserve controller drafts and action guards.
The centered Settings repository picker composes the existing discovery list and
repository-open action, using a compact ghost trigger. It requires explicit
selection from Projects, retains the Settings section across a switch, avoids
reopening the current repository and blocks switching while drafts or pending
work would be lost. The selected repository and Settings refresh use existing
controller transitions; no new server route or generated contract is introduced.
The Repository empty-state picker uses that same transition and guarded
discovery list. Until its route explicitly selects the displayed repository, the
page hides repository details and subsection actions. Repository-type chips use
the retained native/Git-backed profile and grant no additional authority.
Settings identifiers use public Design Core CopyableValue with complete native
values unchanged; abbreviation affects display only. Repository, Search
generation and Sync frontier values share this control. The sidebar Back action
invokes the existing transient Settings close callback without a new read or
persisted Settings-route write. Overview insets, full-width file editor tokens,
a stationary review header and native record Copy submenus are
presentation-only. Clipboard values, read sessions, routes, capabilities and
publication callbacks retain their existing contracts; these layouts add no
request or generated schema. Compact header insets and Source syntax
highlighting remain presentation-only; recorded and draft bytes, safe Markdown
previews and edit publication are unchanged. Highlighting uses detected filename
languages and static token styles under the existing production asset policy.
