# Explicit native Git bridge

The existing CliRunner dispatch hook is the integration seam for the staged
[Git interoperability](../git-interop/index.md) capability. Backend discovery
opens the workspace through JJ's loader without snapshotting or reconciling
operations. Ordinary Git repositories and creation commands retain upstream
dispatch. Native import/export remains unsupported; implemented stages alone
appear in the capability matrix. Aliases and global arguments are parsed
upstream.

Native Git bindings live in private device-local state outside checkouts. An
explicit remote add/set-url approves one HTTPS/SSH or canonical local endpoint;
replicated logical names grant no transport permission. A binding fingerprint
changes with endpoint approval and prevents lease reuse across destinations.
Cache attempts are private, exclusively locked and disposable, with bounded Git
subprocess output, duration and disk use. Hooks and repository-provided helpers
remain disabled. Fetch uses one explicit scope and the existing native
transaction/rebase/workspace finish path with a pending typed Git delta.
Unsupported parsed flags and historical/non-integrating mutation contexts fail
before transport. Ordinary Git-backed dispatch remains upstream-owned.

Selected native push freezes bookmark targets and exact destination leases,
retains representations and intent before execution, then records protocol
outcomes and applies remote bookkeeping without moving local bookmarks. Dry-run
does not publish an intent or bridge-induced code. `jj store git status` reads
durable attempts; `jj store git recover` inspects the original approved endpoint
and repairs local bookkeeping without retransmitting. Capability gates remain
explicit for conflicts, tag publication, sign-on-push and unsupported push
options. Bookmark selectors and atomic multi-ref transport are qualified below.

Native Git code repositories use reducer-2 origin negotiation for data-only
facts. Device bindings are still explicit and local. The bridge capability
reports GB6 qualification on macOS ARM64 with JJ 0.45.1, gix 0.87.1 and Git
2.55.0 under `jj-git-sha1-0.45.1-v1`. Native fact replication is data-only;
ordinary upstream dispatch is unchanged. Linux/Windows and external forge
deployment qualification remain deferred.

Native Git bookmark selection supports patterns, --revision, --all, --tracked,
--deleted, --change and --named under the bookmark-only capability profile.
Generated bookmark preparation is a separate accepted native operation before
intent preparation, and dry-run performs no such publication. Multiple selected
updates use atomic transport with exact leases for every full ref. Remote rename
and removal change owned alias/tracking metadata while preserving durable facts.
Unsupported tag publication, signing/conflict profiles and push options still
reject before external writes; ordinary backend command semantics are unchanged.

The Git transport supervisor uses an internal executable entry point before
CliRunner and native credential loading. It accepts invocation-owned argv and a
lifetime pipe; replicated Git facts cannot launch it. Hard cancellation closes
the pipe and terminates the owned Git group without changing native evidence.

Git apply/recovery reads the frozen native operation's remote-ref base and
merges its result into current tracking state, including renamed aliases. An
attempt cache starts with owner-only directory permissions and remains safely
removable after a hard-killed invocation.

Native Git honors explicit absolute device askpass/SSH executable and CA-file
approvals. `JJ_STORE_GIT_CREDENTIAL_HELPER` selects a supported OS helper or
`none`. Configuration and TLS-verification environment injection stays disabled;
helpers do not inherit native bearer or server storage credentials.

The GB6 cold fixture uses production CLI/server binaries, persistent embedded
RustFS, actual Git refs and a freshly built served browser. It retains a native
review anchor and AgentStore receipt through cache deletion, server restart, Git
publication and external merge import. Git hosting outages leave ordinary local
new/describe usable; no Git checkout or background mirror is installed.

The Projects hub is browser presentation over the existing local-service
WorkStore model. The registered HTTP capture housed in the CLI lifecycle suite
also verifies browse summaries and generated response contracts. Native task
creation still requires the local service and does not require an upstream.
Server-free storage remains deferred.
