# Bundled application documentation

JST-DOC-003 adds installation-level Help above the profile footer. The
full-viewport modal embeds the bundled site, traps focus, handles Escape from
its same-origin frame and returns focus on dismissal. The underlying application
remains mounted, preserving route, drafts and pending commands. Docs page/scroll
state remains session-only.

Initial Help loads show a labelled skeleton matching the manual's header,
navigation/content/contents columns, semantic palette and responsive rail
breakpoints. Pending page navigation keeps the real chrome and rails with a
content skeleton rather than inline loading text. Failed loads publish the
actual bounded response or readiness error through the application notification
owner, with an explicit Retry action. Retry retains dismissal until successful
recovery; no duplicate inline error is shown. Closing Help preserves the
underlying route and drafts.

Internal page navigation retains the iframe document, header, search runtime,
theme and fonts. Same-origin bundled pages replace their validated content and
navigation regions, resolve relative assets against the requested page and
reinitialize Material's document lifecycle. Back/Forward restores page and
scroll; aborted or superseded reads cannot replace newer content. External and
modified links retain ordinary browser behavior. Loading/error/ready messages
are admitted only from the exact same-origin Help frame.

The Help entry follows primary sidebar navigation geometry: a centered 2rem icon
button with 0.5rem horizontal rail padding when collapsed, and a full-width
labelled button inset by 0.75rem when expanded. Connection-screen Help uses a
fixed 5rem button at the trailing edge of the card footer, aligned with the
content inset at normal and narrow widths.

The generated manual uses the app's semantic palette, local DM Sans and Source
Code Pro fonts, typography scale, sidebar chrome and rounded content surfaces.
The same-origin frame observes only the host's appearance attributes and glass
opacity, including previews and cross-tab/system updates. Theme changes preserve
the current documentation page, scroll, search and the application's drafts.
Embedded Help hides its independent palette selector; the standalone site keeps
a light/dark switch using the Original palette. Light/dark Mermaid images follow
the resolved color mode. The navigation rails fill their allotted widths without
fixed inner padding; groups initially collapse on the landing page and reveal
the selected page's path after navigation.

Help's sidebar links, section hover and Close button use the app sidebar's
theme-derived interaction treatments, including prominent Buzz selections.
Overlay/content separators use a subtle foreground tint rather than a neutral
grey border. The sticky documentation header retains an opaque palette surface
so scrolling content cannot show through it, including with glass/Buzz chrome.

Repository Files render the selected row window before commit headers, per-path
attribution and landing documents. Those independent reads retain the explicit
repository/operation/commit selection, cancel obsolete work, and cannot replace
newer rows or make a usable directory fail. Load at most two enrichment reads
concurrently; only displayed windows request attribution, and repeated document
IDs reuse already loaded content. Row metadata comes from its own attribution,
never by copying the selected commit to every row. Loading, unavailable and
failed optional metadata stay distinct from the file listing's status.

Repository startup loads only the first bookmark and workspace windows. It never
drains references to discover working copies. Workspace continuation is an
explicit user action with its own captured snapshot. Route restoration resolves
selected bookmark/workspace names directly, so a name outside the loaded first
page remains usable. Explicit commit Files, Agents and Reviews content are not
blocked by the bookmark picker; optional reference failures retain usable
content.

Git-backed repositories use the same graph preparation action when their local
capability advertises it. A rebuild-required local response retains the exact
operation and registration generation that failed, even if the current heads or
route inputs advance. Starting and status observation use only the local task
endpoints; responses must match the source repository, registration and original
operation. Completion retries the pinned view and leaves files/reference
navigation available. Local task state cannot fabricate native epoch or
cache-report data.

Graph preparation retains one exact task and serializes start, status and cancel
requests. Healthy observation waits for the exact task's event, including events
received before the start response. During an outage only, status observation
backs off from 500 ms to five seconds, has no overlapping requests, and rechecks
the active read scope before every request. Navigation, disconnect and teardown
abort obsolete observations and clear the fallback timer; this does not claim
that the server worker was canceled. Both the recovery action and Repository
settings expose Cancel once a task is known. A cancel request keeps preparation
pending until the server reports ready, failed or canceled; transport errors
leave cancellation unconfirmed, retain the exact task and allow an explicit
retry. Cancel never starts or retargets preparation. A confirmed cancel keeps
the original recovery condition available without presenting it as an unexpected
failure.

Automatic repository refresh queues updates while preparation is being observed,
so an incoming event or fallback reload cannot discard the retained task.
Explicit navigation still stops obsolete observations; terminal preparation
releases the refresh guard.
