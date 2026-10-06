# Browsing UI execution

Review detail and Projects Activity consume the existing authorized native read
routes without adding repository authority. Exact review anchors determine code
comparisons; related agent context comes only from root-pinned associations and
sanitized session details. Project/task activity reads independently authorized
linked repositories at explicit code operations, preserves bounded continuation
snapshots, and treats missing capture or read failures as unavailable context.
Activity tab routes and the validated all/agent/unassisted classification
survive hash/history restoration. Classification uses bounded root-pinned
association metadata and does not load sanitized session bodies; unresolved
metadata remains unavailable, not evidence of an unassisted commit. The
repository Projects route browses authorized metadata homes with an explicit
linked-repository query while retaining its independent code operation and
repository navigation. Cache scope includes the target repository. Activity tab
routes survive hash/history restoration. Native commit links from Projects
authorize a code connection even when no code source is active, retain the exact
operation and restore the Projects scope with Back. Task sections initially
expand only Description and main-tab skeletons use a page inset.

Initial browser placeholders cover connection and repository discovery as well
as the individual Projects, repository-tab and task/review detail reads. These
are presentation states over the existing asynchronous controller; they add no
requests or read authority. Previously displayed snapshots remain visible during
background refresh, with empty/error/unsupported states shown after reads
settle.

Repository sidebar Threads use the existing authorized, bounded Threads list
endpoint independently from the selected code source. Each entry retains its own
native repository and stable thread ID; selecting it connects that owner before
opening the thread. Memory-only directories are cleared with the credential
session, discard late or foreign-repository pages, retain loaded windows on
refresh and omit unsupported local folders. This presentation adds no endpoint
or persisted contract. Browse threads uses the connected native repository or an
authorized native fallback; repository/Threads breadcrumbs preserve the
available code-operation context.

A selected JJ Log or Branches change exposes a read-only Evo Log action in the
right-rail Actions section. Its separate resizable column reads the existing
native evolution graph at the displayed operation, preserves server predecessor
edges and shows incomplete, empty, loading and failed states. The full anchor
commit is stored as `evolution` in the hash; opening, closing and selecting a
history row preserve the main commit, workspace and typed mutation target.
Snapshot-checked opaque pagination deduplicates identities, rejects repeated
cursors and caps displayed nodes at 1,000. Browser history and reload restore
the column; page, main revision, bookmark or workspace changes close it.
Repositories navigation sections have no top margin in expanded or compact form.
The conflict sheet header has a bottom border separating its actions from the
body.

The LocalTrusted application reuses the reviewed Storybook shell for Files,
Branches, JJ Log, Op Log, evolution, context-aware right-rail details, typed
mutations and native conflict editing. Branch bookmark cards stay square through
the final visible row; the branch command view container does not round that row
and its unselected hover matches JJ Log's neutral row hover. Bookmark selection
exposes exact metadata and branch-capable actions before a revision is selected;
revision selection replaces that bookmark detail. The rail uses a page-specific
Community heading and create action instead of a duplicate Details close header.
A selected change, operation or bookmark exposes its existing capability-gated
Actions section below Details navigation. The section follows the exact loaded
selection and disappears when that context is cleared. Operation selection
exposes reconciliation only for multiple heads and omits Edit description and
Abandon change commands. This is presentation only; typed mutation previews,
authorization and confirmation flows remain unchanged. It bootstraps the
installation catalog and opens the sole repository without credentials. An
explicitly configured LocalTrusted search entry starts the existing background
Tantivy projection through a stable installation-derived read-only principal;
repository capabilities expose the command only when that index is configured.
M4 registry-only dependency finalization and shared browser release
qualification are deferred. The authenticated browser supports one memory-only
manual token and bounded repository discovery without adding an account or OIDC
layer.

The Files controller fetches the selected commit independently from graph
history so the production header can show the last author avatar, commit
message, Git revision, JJ change ID and commit time. Mutation prose uses
Community's form composer. A fixed Edit description or Abandon target is
presented as one compact row with author avatar, existing description, copyable
Git/JJ identities and relative time rather than an editable or disabled raw ID
input. The same presentation includes bookmark and workspace chips; selectable
Create change, Create bookmark and Move bookmark targets open a searchable
Design Core Command View over loaded revisions and retain the selected full
native commit ID in the typed request. The selector aligns avatar and content
vertically for descriptionless commits and uses compact oval GIT/JJ labels in
its selected button and options. Their badge widths match so identifier text
aligns. Those labels are 10px, one pixel larger than the revision-ID copy-button
labels. A fixed identifier column stacks Git above JJ and keeps their chips
aligned across options, reserving the second row for missing change IDs;
relative commit time sits under the title at the same left edge, with
bookmark/workspace chips alongside it on that row. This styling leaves the bound
identities and mutation commands unchanged. Collaboration presentation uses the
Community message/thread/composer models and Design Core tool/diff primitives.
Agent session cards show the integration's agent avatar and name, state,
captured start time and compact captured duration when evidence supplies both
endpoints. SHA-shaped agent, review and anchor identities use copy controls;
tool details can embed an exact loaded patch or sanitized old/new edit input.
Revision and path selections open route-addressable Agent context and Review
discussion columns, and an agent header links back to its associated change.

The browser derives integration avatars from bundled, reviewed agent artwork;
agent identities keep Community's rounded-square shape while people and device
authors remain circular. Git author email is normalized before the same Gravatar
identicon lookup used by Community. Collaboration summaries may group the agent
and review participants already proven by the loaded root-pinned context, but do
not infer associations that the collaboration APIs did not return. Exact
code-text or code-artifact review anchors may load their immutable comparison
through the existing pinned diff reads so the discussion can render the comment
above its inline file diff.

The production diff controller passes Design Core line accessories and
annotations through its single-file wrapper. Creating a line comment resolves
the selected before or after file object at its explicit comparison operation,
verifies the displayed line text, derives the UTF-8 byte interval and digest,
and sends one typed `create_thread` command. Tool reply actions bind
`agent_artifact` anchors to the loaded session and tool snapshots, using the
tool-call identity as the V1 attempt identity when capture supplies no separate
attempt. A committed receipt refreshes summaries, selects and reloads the exact
returned thread, and renders it inline; `outcome_unknown` retains the stable
request ID for the existing status route.

Local workspace validation may resolve `@lapismd/design-core` from the sibling
source repository through the guarded pnpm hook. That development resolution
does not change browser request contracts, static-asset policy or runtime
transport behavior. Clean environments without the sibling source use the
registry range and committed registry lockfile.

Real browser deployment acceptance is a separate opt-in lane from captured
response tests. It resets an isolated OS-temporary LocalTrusted installation,
starts the built bundle and native server with embedded RustFS, auto-opens the
sole catalog repository without credentials, publishes through another native
CLI process over the owner-only Unix socket, and requires explicit refresh
before the displayed operation changes. The selected bookmark, commit and change
identities must survive that refresh. Its environment-selected fixture root and
cleanup cannot alter the interactive `.local/dev-demo` installation.

The disposable repository used by this lane contains a native unresolved
multi-parent file conflict and a concurrent rewrite pair with one JJ change ID
and two commit IDs. The browser consumes the server-provided identities and
edges for both cases; seeding them does not add an API-only ancestry or conflict
representation.
