# Interactive agent execution

Explicit JJ_STORE_ACP_EXECUTABLE selects a rebuilt harness outside checkout
configuration. The CLI composition supplies the fixed native-aware JJ executable
and workspace provider. The LocalTrusted service supervises ACP v2 sessions and
drains them before storage shutdown. Native and local workspace execution retain
separate acceptance gates.

The application development launchers select `desktop/resources/jj-acp` by
default, matching the desktop's bundled-resource startup. `dev:restart`,
`dev:ui` and `dev:backend:restart` supply the selected ACP path and paired
custom `jj` on every launch; availability must not depend on a prior shell
export. Explicit trusted executable overrides retain precedence. Before
replacing a running development service, preflight requires regular executable
files and ACP help advertising control version 3. Missing or stale resources
fail with staging guidance instead of silently starting without agent
management. The inspection only requests help; provider authentication,
saved-agent Start/Stop state and explicit Send semantics remain unchanged. Bare
server invocations retain explicit operator environment configuration.

`JJ_STORE_AGENT_JJ_EXECUTABLE` optionally selects the paired custom `jj`; the
default is the `jj` beside the server binary. The hidden `jj agent-workspace`
command takes full operation and revision IDs, an owned name and an absolute
destination. It loads source metadata without snapshotting the source working
copy and checks out only the destination. Creation recovery verifies its parent
and tree. A durable ready phase precedes opening an adapter; incomplete
preparation cannot resume or continue execution.

Scanner preparation runs once per supervisor in the background. Status reports
preparing, ready or unavailable; launches fail Busy before recording dispatch
when privacy preparation is incomplete. Path rebinding reuses immutable scanner
rules. `JJ_STORE_AGENT_PRIVACY_CACHE` is trusted launch configuration and must
point outside a checkout. No browser body supplies these values.

Shutdown stops publication scheduling before closing agents. Process closure is
bounded at twenty seconds; final publication gets three seconds. Canceled
publication retains the exact spool intent and unpublished sequence. Restart
retries publication receipts, never agent prompts.

JST-CLI-004 remains unchanged for managed-identity presentation. Rebuilt
explicit development binaries may replace the owned development backend while
retaining Vite and its fixture. Read-only capture-alias and workspace-proof
projections introduce no CLI command, executable selection, installed-tool
replacement or automatic execution. The ready workspace operation may differ
from the original launch pin before its first turn; neither value is inferred by
the client.

Application-managed Codex tooling resolves before ambient PATH and conventional
device tool directories, using executable-bit checks. The private selected bin
and its resolved Node are supplied to jj-acp for both launch and resume.
Installs never replace global tools, authenticate, restart sessions or change
jj-acp's no-network/no-auto-install contract. Shutdown aborts the bounded
installer process group before storage teardown. Only the explicit Settings
install command may fetch the fixed packages; ordinary development restarts
preserve selection.

M24 adds
[reusable agent conversations](../collaboration/37-reusable-agent-conversations.md)
under JST-COLLAB-034 through JST-COLLAB-036. Existing M23 qualification remains
unchanged.

The supervisor publishes semantic installation freshness after privacy
preparation, visible journal settlement and worker transitions. Repeated private
bookkeeping with unchanged visible state stays quiet. This event integration
preserves JST-CLI-004 executable selection, saved Start/Stop state, explicit
Send and bounded shutdown; no CLI mutation or prompt replay is introduced.
