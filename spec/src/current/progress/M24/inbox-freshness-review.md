# Inbox read lifecycle review

E-INBOX-FRESHNESS-001 belongs to the enclosing default-workspace change
`klrylvzw`. M24 and M25 remain **In progress**. Visual baselines remain
unapproved. This change qualifies browser read ownership and request reuse;
native publication, provider, permission, cancellation and platform gates retain
their independent evidence.

The inbox shares installation status, defaults, targets and feedback reads. The
shell runtime observer also uses retained captures: rerunning that observer or
remounting the shell cannot force all installation reads. Its request-count
regression includes route-reactive availability and the actual shell observer.
Returning to an unchanged session reuses captured history and resumes after its
last sequence. Advertised application freshness replaces the selected inbox's
extra activity stream. Closed conversation details and sidebar groups perform no
background reads. Hidden changes retain dirtiness. Exact saved-change
comparisons have a bounded observer cache. Current Threads rows also feed their
sidebar, avoiding a second read after creation.

The audit covers the inbox, agent directories, runtime/compute Settings,
conversation metadata, native/local Threads, project and repository navigation,
capture/folder Settings and shared application refresh. Healthy idle and feed
outage no longer start periodic view-data polling in these automatic refresh
owners. Explicit preparation paths still contain bounded status observations:
`agent-revision-index.ts`, `http-blame.ts`, `production-index-preparation.ts`,
`production-search-settings.ts` and the outage fallback in
`freshness-journal.ts`. Removing those needs complete task event coverage,
including kind-index progress, and separate focused qualification. Reconnect,
dirty-read retry, debounce, timeout and cache eviction timers are not idle data
polls.

Live measurements separate fast responses from long waits before connecting.
Earlier transcript captures took 47.6 seconds, including 11.5/34.4-second
connection waits and 29/45-millisecond response waits. Later fresh loads also
showed connection queueing. Its precise owner is not established; no missing
database index or leaked stream is claimed. Warm navigation verifies retained
captures independently from that unresolved first-load latency. HMR resets are
excluded from warm-cache claims. A stable a969/ddcc navigation cycle showed zero
API reads and retained 7/11 rendered message articles without a loading
indicator. The first return exposed a cold-start race: history was cached under
an unknown publication before status arrived. Its exact regression failed with
two reads instead of one; captured event commitment now supplies that initial
boundary. Later publication changes still recapture. A subsequent first ddcc
capture waited 4.98 seconds before connecting, with 21-48ms transcript response
waits; this does not qualify the independent connection queueing as resolved.

Request-count, cancellation, event-during-read, hidden-view, failure retention,
Unicode/history and browser interaction coverage is recorded in the evidence.
Six architecture oversizes are byte-identical to the parent revision. The
inherited repository navigation story expected the standalone Agents button to
be absent; its assertion now agrees with JST-UI-015. Fixture staging no longer
depends on duplicate boot reads. Existing ResizeObserver diagnostics are not
qualified as resolved by this data-lifecycle change.

## Cross-instance connection follow-up — 2026-10-10

E-EVENT-CONNECTIONS-001, in enclosing default change `zswrwonw`, identifies
HTTP/1 connection-slot exhaustion: five unfinished application streams and one
legacy agent stream belonged to the Codex browser network process. Requests
waited before connecting, while transcript responses took tens of milliseconds.
This establishes the transport bottleneck, not a missing database index or the
exact origin of every old background view. The prior cache repair remains valid.

Same-asset, same-credential app instances now share a bounded worker transport.
Sixteen owners in two fresh real browser tabs used one connection; a fixture
event reached all sixteen. Closing one tab retained the stream, final release
closed it, and ordinary request probes took 2.6–29.4ms. The fixture tests actual
built worker lifecycle under same-origin CSP, separate from live-repository
latency. Focused protocol/controller and catalog coverage passes. Three wider
web test failures reproduce on the unchanged parent and remain recorded.

The live browser still includes older application/legacy streams outside the two
tabs accessible to this task. Reloading only those two does not retire the old
connections. A clean live chat/files after comparison remains pending until
those views reload; no sub-second live result is claimed. Source edits and UI
restarts preserve backend PID 92799 and the fixture. M24/M25 remain In progress;
visual baselines and ResizeObserver diagnostics are not qualified as resolved.

Next: re-time both uncached live actions after older views reload, qualify
explicit preparation flows against complete task events, and retain the
independent milestone gates.
