<script lang="ts">
  import { onDestroy } from "svelte";
  import { displayable } from "./lib/graph";
  import GraphView from "./lib/Graph.svelte";
  import TextPreview from "./lib/TextPreview.svelte";
  import { ApiError, Client, fullId, pretty } from "./lib/api/client";
  import type {
    Commit,
    Envelope,
    Graph,
    Head,
    Model,
    Snapshot,
    Task,
    Tree,
  } from "./lib/api/client";

  let repository = $state("");
  let credential = $state("");
  let client = $state<Client | null>(null);
  let head = $state<Head | null>(null);
  let snapshot = $state<Snapshot | null>(null);
  let operation = $state("");
  let graph = $state<Graph | null>(null);
  let cursor = $state<string | null>(null);
  let tab = $state<"log" | "operations" | "evolution">("log");
  let revset = $state("all()");
  let workspace = $state("");
  let email = $state("");
  let anchor = $state(Date.now());
  let evolution = $state("");
  let latest = $state<{ epoch: string; position: number | bigint } | null>(
    null,
  );
  let busy = $state(false);
  let error = $state("");
  let stale = $state(false);
  let live = $state("Disconnected");
  let selected = $state("");
  let detail = $state<Commit | Model<"OperationDetail"> | null>(null);
  let tree = $state<Tree | null>(null);
  let treeCursor = $state<string | null>(null);
  let treeSnapshot: Snapshot | null = null;
  let graphRequest: {
    kind: typeof tab;
    query: Record<string, string | number | bigint | boolean | undefined>;
    more: boolean;
  } | null = null;
  let path = $state("");
  let content = $state("");
  let contentTitle = $state("");
  let detailBusy = $state(false);
  let conflicts = $state<Model<"ConflictResponse"> | null>(null);
  let differences = $state<Model<"FileDiffResponse"> | null>(null);
  let compareFrom = $state("");
  let opDiff = $state<Model<"OperationDiffResponse"> | null>(null);
  let neededIndex = $state<Snapshot | null>(null);
  let task = $state<Task | null>(null);
  let control = new AbortController();
  let inspectControl = new AbortController();
  let feedControl = new AbortController();
  let job = 0;
  let inspection = 0;
  let fileJob = 0;
  let directoryJob = 0;
  let diffJob = 0;
  let fileControl = new AbortController();
  let directoryControl = new AbortController();
  const short = (id: string) => id.slice(0, 12);
  const preview = (value: unknown) => {
    const text = pretty(value);
    return text.length > 262144
      ? `${text.slice(0, 262144)}\n[Preview truncated at 256 KiB]`
      : text;
  };
  function clearDetail() {
    fileJob++;
    directoryJob++;
    diffJob++;
    fileControl.abort();
    directoryControl.abort();
    inspectControl.abort();
    inspectControl = new AbortController();
    inspection++;
    selected = "";
    detail = null;
    tree = null;
    treeSnapshot = null;
    treeCursor = null;
    conflicts = null;
    differences = null;
    content = "";
    contentTitle = "";
    opDiff = null;
    path = "";
    compareFrom = "";
    detailBusy = false;
  }
  function disconnect(message = "") {
    control.abort();
    feedControl.abort();
    client?.close();
    client = null;
    credential = "";
    latest = null;
    head = null;
    snapshot = null;
    graph = null;
    cursor = null;
    operation = "";
    neededIndex = null;
    task = null;
    graphRequest = null;
    clearDetail();
    job++;
    busy = false;
    stale = false;
    live = "Disconnected";
    error = message;
  }
  onDestroy(() => disconnect());
  function report(cause: unknown) {
    if (cause instanceof DOMException && cause.name === "AbortError") return;
    if (cause instanceof ApiError) {
      if (cause.code === "index_rebuild_required")
        neededIndex = cause.snapshot ?? null;
      if (cause.heads) head = cause.heads;
      error = cause.message;
    } else
      error =
        "The request could not be completed. Retry or select a new snapshot.";
  }
  function checkPin(
    value: Snapshot,
    expected = snapshot?.operation_id ?? operation,
  ) {
    if (
      value.repo_id !== client?.repository ||
      value.epoch !== head?.epoch ||
      value.operation_id !== expected
    )
      throw new ApiError("snapshot_mismatch");
  }
  async function connect(event: SubmitEvent) {
    event.preventDefault();
    const secret = credential;
    disconnect();
    control = new AbortController();
    const version = ++job;
    try {
      const active = new Client(repository.trim(), secret, () => {
        if (client === active)
          disconnect(
            "Access expired or was revoked. Connect with a current token.",
          );
      });
      client = active;
      busy = true;
      await active.request<Model<"Capabilities">>(
        "capabilities",
        {},
        { signal: control.signal },
      );
      const captured = await active.request<Head>(
        "snapshot",
        {},
        { signal: control.signal },
      );
      if (client !== active || version !== job) return;
      if (
        captured.repo_id !== active.repository ||
        captured.version !== 1 ||
        !Array.isArray(captured.heads)
      )
        throw new ApiError("invalid_snapshot");
      head = captured;
      operation = captured.heads.length === 1 ? captured.heads[0] : "";
      busy = false;
      feedControl = new AbortController();
      void follow(active, feedControl.signal);
      if (operation) await loadGraph();
    } catch (cause) {
      if (version !== job) return;
      if (client) {
        client.close();
        client = null;
      }
      busy = false;
      report(cause);
    }
  }
  async function refresh() {
    if (!client) return;
    const active = client;
    control.abort();
    control = new AbortController();
    const version = ++job;
    busy = true;
    try {
      const fresh = await active.request<Head>(
        "snapshot",
        {},
        { signal: control.signal },
      );
      if (client !== active || version !== job) return;
      head = fresh;
      operation = fresh.heads.length === 1 ? fresh.heads[0] : "";
      stale = false;
      graph = null;
      snapshot = null;
      cursor = null;
      latest = null;
      clearDetail();
      feedControl.abort();
      feedControl = new AbortController();
      void follow(active, feedControl.signal);
      if (operation) await loadGraph();
    } catch (cause) {
      if (client === active && version === job) report(cause);
    } finally {
      if (version === job) busy = false;
    }
  }
  async function follow(active: Client, signal: AbortSignal) {
    let after: string | undefined;
    let delay = 500;
    while (!signal.aborted && client === active) {
      try {
        live = "Live";
        let reset = false;
        await active.events(after, signal, (event, id) => {
          if (signal.aborted || client !== active) return;
          after = id;
          delay = 500;
          latest = {
            epoch: event.cursor.epoch,
            position: event.cursor.position,
          };
          if (
            snapshot &&
            (event.cursor.epoch !== snapshot.epoch ||
              BigInt(event.cursor.position) > BigInt(snapshot.published_seq))
          )
            stale = true;
          if (event.type === "reset") {
            reset = true;
            head = event.snapshot;
            stale = true;
            live = "Refresh needed";
          } else if (event.type === "ready") head = event.snapshot;
        });
        if (reset) return;
      } catch {
        if (signal.aborted || client !== active) return;
      }
      live = "Reconnecting";
      await new Promise<void>((resolve) => {
        const timer = setTimeout(done, delay);
        function done() {
          clearTimeout(timer);
          signal.removeEventListener("abort", done);
          resolve();
        }
        signal.addEventListener("abort", done, { once: true });
      });
      delay = Math.min(delay * 2, 10000);
    }
  }
  async function loadGraph(more = false, retry = false) {
    if (!client) return;
    control.abort();
    control = new AbortController();
    const version = ++job;
    const active = client;
    busy = true;
    error = "";
    neededIndex = null;
    task = null;
    if (!more && !retry) {
      graph = null;
      snapshot = null;
      cursor = null;
      clearDetail();
      anchor = Date.now();
    }
    try {
      const query: Record<
        string,
        string | number | bigint | boolean | undefined
      > =
        (more || retry) && graphRequest
          ? {
              ...graphRequest.query,
              cursor: more ? (cursor ?? undefined) : graphRequest.query.cursor,
            }
          : {
              at_op: fullId(operation),
              limit: 50,
              cursor: more ? (cursor ?? undefined) : undefined,
              ...(tab === "log"
                ? {
                    revset,
                    workspace: workspace || undefined,
                    user_email: email,
                    at_time_millis: anchor,
                  }
                : {}),
              ...(tab === "evolution" ? { commit: fullId(evolution) } : {}),
            };
      const kind = (more || retry) && graphRequest ? graphRequest.kind : tab;
      graphRequest = { kind, query, more };
      const value = await active.request<Envelope<Graph>>(kind, query, {
        signal: control.signal,
      });
      if (version !== job || active !== client) return;
      checkPin(value.snapshot, String(query.at_op));
      if (value.data.data.graph !== kind) throw new ApiError("graph_mismatch");
      let nextGraph = value.data;
      if (more && graph) {
        if (pretty(snapshot) !== pretty(value.snapshot))
          throw new ApiError("pagination_snapshot_mismatch");
        nextGraph = {
          ...value.data,
          data: {
            graph: kind,
            nodes: [...graph.data.nodes, ...value.data.data.nodes],
          },
        } as Graph;
      }
      if (!displayable(nextGraph))
        throw new ApiError("graph_display_limit_narrow_the_query");
      graph = nextGraph;
      snapshot = value.snapshot;
      cursor = value.next_cursor;
      stale = Boolean(
        latest &&
        (latest.epoch !== snapshot.epoch ||
          BigInt(latest.position) > BigInt(snapshot.published_seq)),
      );
    } catch (cause) {
      if (version === job) report(cause);
    } finally {
      if (version === job) busy = false;
    }
  }
  async function changeTab(next: typeof tab) {
    tab = next;
    graph = null;
    cursor = null;
    clearDetail();
    if (tab !== "evolution" || evolution) await loadGraph();
  }
  async function select(id: string) {
    if (!client || !snapshot) return;
    clearDetail();
    selected = id;
    detailBusy = true;
    const version = inspection;
    const active = client;
    const at_op = snapshot.operation_id;
    const signal = inspectControl.signal;
    try {
      if (tab === "operations") {
        const value = await active.request<Envelope<Model<"OperationDetail">>>(
          `operations/${fullId(id)}`,
          {},
          { signal },
        );
        if (version !== inspection || active !== client) return;
        checkPin(value.snapshot, id);
        detail = value.data;
        const diff = await active.request<
          Envelope<Model<"OperationDiffResponse">>
        >(`operations/${id}/diff`, {}, { signal });
        if (version === inspection && active === client) opDiff = diff.data;
      } else {
        const value = await active.request<Envelope<Commit>>(
          `commits/${fullId(id)}`,
          { at_op },
          { signal },
        );
        if (version !== inspection || active !== client) return;
        checkPin(value.snapshot, at_op);
        detail = value.data;
        compareFrom = value.data.commit.parents[0] ?? "";
        await directory("");
        const conflict = await active.request<
          Envelope<Model<"ConflictResponse">>
        >(`commits/${id}/conflicts`, { at_op }, { signal });
        if (version === inspection && active === client) {
          checkPin(conflict.snapshot, at_op);
          conflicts = conflict.data;
        }
      }
    } catch (cause) {
      if (version === inspection) report(cause);
    } finally {
      if (version === inspection) detailBusy = false;
    }
  }
  async function directory(next: string, more = false) {
    if (!client || !selected || !snapshot) return;
    const version = inspection;
    const directoryVersion = ++directoryJob;
    const active = client;
    directoryControl.abort();
    directoryControl = new AbortController();
    try {
      const value = await active.request<Envelope<Tree>>(
        `commits/${selected}/tree`,
        {
          at_op: snapshot.operation_id,
          path: next,
          limit: 50,
          cursor: more ? (treeCursor ?? undefined) : undefined,
        },
        {
          signal: AbortSignal.any([
            inspectControl.signal,
            directoryControl.signal,
          ]),
        },
      );
      if (
        version !== inspection ||
        directoryVersion !== directoryJob ||
        active !== client
      )
        return;
      checkPin(value.snapshot);
      if (more && pretty(treeSnapshot) !== pretty(value.snapshot))
        throw new ApiError("pagination_snapshot_mismatch");
      treeSnapshot = value.snapshot;
      tree =
        more && tree
          ? { ...value.data, entries: [...tree.entries, ...value.data.entries] }
          : value.data;
      path = next;
      treeCursor = value.next_cursor;
    } catch (cause) {
      if (version === inspection && directoryVersion === directoryJob)
        report(cause);
    }
  }
  async function inspect(entry: Model<"TreeEntry">) {
    const value = entry.terms.length === 1 ? entry.terms[0] : null;
    if (value?.type === "tree") {
      await directory(path ? `${path}/${entry.name}` : entry.name);
      return;
    }
    contentTitle = entry.name;
    content = "Loading…";
    const version = inspection;
    const fileVersion = ++fileJob;
    const active = client;
    fileControl.abort();
    fileControl = new AbortController();
    try {
      if (value?.type === "file" && active) {
        const data = await active.file(
          value.id,
          snapshot!.operation_id,
          AbortSignal.any([inspectControl.signal, fileControl.signal]),
        );
        if (
          version !== inspection ||
          fileVersion !== fileJob ||
          active !== client
        )
          return;
        content = `${data.range ? `Byte range ${data.range}\n\n` : ""}${data.text ?? `Binary preview (${data.bytes} bytes fetched)\n${data.hex}`}`;
      } else content = preview(entry);
    } catch (cause) {
      if (version === inspection && fileVersion === fileJob) {
        content = "";
        report(cause);
      }
    }
  }
  async function compare() {
    if (!client || !snapshot) return;
    const active = client;
    const version = inspection;
    const diffVersion = ++diffJob;
    try {
      const value = await active.request<Envelope<Model<"FileDiffResponse">>>(
        "diff",
        {
          at_op: snapshot.operation_id,
          from: fullId(compareFrom),
          to: fullId(selected),
          content: true,
        },
        { signal: inspectControl.signal },
      );
      if (
        version === inspection &&
        diffVersion === diffJob &&
        active === client
      ) {
        checkPin(value.snapshot);
        differences = value.data;
      }
    } catch (cause) {
      if (version === inspection && diffVersion === diffJob) report(cause);
    }
  }
  async function compareOperation(parent: string) {
    if (!client) return;
    const active = client;
    const version = inspection;
    try {
      const value = await active.request<
        Envelope<Model<"OperationDiffResponse">>
      >(
        `operations/${selected}/diff`,
        { parent },
        { signal: inspectControl.signal },
      );
      if (version === inspection && active === client) opDiff = value.data;
    } catch (cause) {
      if (version === inspection) report(cause);
    }
  }
  async function prepare() {
    if (!client || !neededIndex) return;
    const active = client;
    const version = job;
    const target = neededIndex.operation_id;
    try {
      const started = await active.request<Task>(
        "index/tasks",
        {},
        {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ operation: target }),
          signal: control.signal,
        },
      );
      if (active !== client || version !== job) return;
      task = started;
      while (task && ["running", "canceling"].includes(task.status)) {
        await new Promise((resolve) => setTimeout(resolve, 500));
        if (active !== client || version !== job) return;
        const result: Task = await active.request<Task>(
          `index/tasks/${encodeURIComponent(task.id)}`,
          {},
          { signal: control.signal },
        );
        if (active !== client || version !== job) return;
        task = result;
      }
      if (task?.status === "ready") {
        error = "";
        neededIndex = null;
        await loadGraph(graphRequest?.more ?? false, true);
      }
    } catch (cause) {
      if (active === client && version === job) report(cause);
    }
  }
  async function cancelTask() {
    if (client && task) {
      const active = client;
      const version = job;
      try {
        const canceled = await client.request<Task>(
          `index/tasks/${encodeURIComponent(task.id)}/cancel`,
          {},
          { method: "POST" },
        );
        if (active === client && version === job) task = canceled;
      } catch (cause) {
        report(cause);
      }
    }
  }
</script>

<svelte:head
  ><meta
    name="description"
    content="Explore native Jujutsu repository history."
  /></svelte:head
>
<header class="masthead">
  <a class="brand" href="/" aria-label="jj-store home"
    ><span class="brand-mark">jj</span> store</a
  ><span class="masthead-label">NATIVE HISTORY</span><span class="live"
    ><span class:online={live === "Live"} class="status-dot"></span>{live}</span
  >
</header>
<main>
  {#if !client}
    <section class="connect-panel" aria-labelledby="connect-title">
      <p class="eyebrow">YOUR REPOSITORY, IN VIEW</p>
      <h1 id="connect-title">Explore native history.</h1>
      <p class="intro">
        Follow commits, operations and the evolution of a change. Connect to a
        repository on this server to begin.
      </p>
      <form onsubmit={connect} class="connect-form">
        <label
          >Repository ID<input
            bind:value={repository}
            placeholder="32-character repository ID"
            required
            autocomplete="off"
            spellcheck="false"
            maxlength="32"
          /></label
        ><label
          >Repository token<input
            bind:value={credential}
            type="password"
            required
            autocomplete="off"
            spellcheck="false"
          /></label
        >
        <p class="muted">Your token stays in memory for this session.</p>
        <button class="primary" disabled={busy}
          >{busy ? "Connecting…" : "Open repository"}<span aria-hidden="true"
            >→</span
          ></button
        >
      </form>
    </section>
  {:else}
    <section class="repo-bar">
      <div>
        <p class="eyebrow">REPOSITORY</p>
        <h1><code>{short(client.repository)}</code></h1>
        <span class="full-id">{client.repository}</span>
      </div>
      <button onclick={() => disconnect()}>Disconnect</button>
    </section>
    <section class="snapshot-bar" aria-label="Snapshot state">
      <span
        ><strong>Position</strong>
        {snapshot?.published_seq.toString() ??
          head?.published_seq.toString() ??
          "—"}</span
      ><span
        ><strong>Operation</strong>
        <code title={snapshot?.operation_id}
          >{snapshot ? short(snapshot.operation_id) : "Not loaded"}</code
        ></span
      ><span><strong>Heads</strong> {head?.heads.length ?? 0}</span><span
        ><strong>Index</strong> {graph?.cache.state ?? "Not loaded"}</span
      ><button class="text-button" onclick={refresh} disabled={busy}
        >Refresh snapshot ↻</button
      >
    </section>
    {#if stale}<div class="notice" role="status">
        Newer history is available. This view remains at its selected snapshot. <button
          onclick={refresh}>Load current heads</button
        >
      </div>{/if}
    <form
      class="snapshot-select"
      onsubmit={(event) => {
        event.preventDefault();
        void loadGraph();
      }}
    >
      <label
        >Selected operation<input
          bind:value={operation}
          placeholder="Full operation ID"
          list="heads"
          spellcheck="false"
          autocomplete="off"
        /></label
      ><datalist id="heads"
        >{#each head?.heads ?? [] as id}<option value={id}
          ></option>{/each}</datalist
      ><button disabled={busy || !operation}>Load operation</button>
    </form>
    {#if head && head.heads.length > 1}<div class="notice">
        <strong>Multiple operation heads.</strong> Select one explicitly.{#each head.heads as id}<button
            class="head-choice"
            onclick={() => {
              operation = id;
              void loadGraph();
            }}>{short(id)}</button
          >{/each}
      </div>{/if}
    <div class="workspace">
      <section class="history" aria-labelledby="history-title">
        <div class="section-heading">
          <div>
            <p class="eyebrow">EXPLORE</p>
            <h2 id="history-title">Repository history</h2>
          </div>
          <span class="count">{graph?.data.nodes.length ?? 0} entries</span>
        </div>
        <nav class="tabs" aria-label="History views">
          {#each [["log", "Commits"], ["operations", "Operations"], ["evolution", "Evolution"]] as [key, label]}<button
              class:active={tab === key}
              aria-pressed={tab === key}
              onclick={() => changeTab(key as typeof tab)}>{label}</button
            >{/each}
        </nav>
        <form
          class="query"
          onsubmit={(event) => {
            event.preventDefault();
            void loadGraph();
          }}
        >
          {#if tab === "log"}<label
              >Revset<input bind:value={revset} spellcheck="false" /></label
            >
            <details>
              <summary>Query context</summary><label
                >Logical workspace<input bind:value={workspace} /></label
              ><label
                >Author email<input bind:value={email} type="email" /></label
              >
            </details>
          {:else if tab === "evolution"}<label
              >Exact commit ID<input
                bind:value={evolution}
                placeholder="Full commit ID"
                spellcheck="false"
              /></label
            >{/if}
          <button disabled={busy || !operation}>Run query</button>
        </form>
        {#if busy}<p class="empty" role="status">
            Loading native history…
          </p>{/if}
        {#if graph}<GraphView
            {graph}
            {selected}
            {select}
          />{#if graph.data.nodes.length === 0}<p class="empty">
              No entries match this view.
            </p>{/if}{:else if !busy}<p class="empty">
            {operation
              ? "Run a query to explore this snapshot."
              : "Choose an operation head to begin."}
          </p>{/if}
        {#if cursor}<button
            class="more"
            onclick={() => loadGraph(true)}
            disabled={busy || (graph?.data.nodes.length ?? 0) >= 1000}
            >Load more at this snapshot</button
          >{/if}
        {#if (graph?.data.nodes.length ?? 0) >= 1000}<p class="muted">
            Display limit reached. Narrow the revset to explore more history.
          </p>{/if}
      </section>
      <section class="inspector" aria-labelledby="inspector-title">
        <div class="section-heading">
          <div>
            <p class="eyebrow">INSPECT</p>
            <h2 id="inspector-title">
              {selected ? short(selected) : "Select an entry"}
            </h2>
          </div>
        </div>
        {#if detailBusy}<p role="status">Loading details…</p>{/if}
        {#if detail && "commit" in detail}
          <p class="description">
            {detail.commit.description || "No description"}
          </p>
          <p class="muted">
            {detail.commit.author.name} · {detail.commit.author.email}
          </p>
          <code class="full-id">{detail.commit.id}</code>
          <button
            onclick={() => {
              evolution = selected;
              void changeTab("evolution");
            }}>View evolution</button
          >
          <h3>Files <span class="muted">/{path}</span></h3>
          {#if path}<button
              onclick={() => directory(path.split("/").slice(0, -1).join("/"))}
              >← Parent directory</button
            >{/if}
          {#if tree}<ul class="file-list">
              {#each tree.entries as entry}<li>
                  <button onclick={() => inspect(entry)}
                    ><span aria-hidden="true"
                      >{entry.terms[0]?.type === "tree" ? "▸" : "·"}</span
                    >{entry.name}<span class="muted"
                      >{entry.conflicted
                        ? "conflicted"
                        : (entry.terms[0]?.type ?? "absent")}</span
                    ></button
                  >
                </li>{/each}
            </ul>
            {#if treeCursor}<button onclick={() => directory(path, true)}
                >More files</button
              >{/if}{/if}
          {#if contentTitle}<h3>{contentTitle}</h3>
            <TextPreview class="source">{content}</TextPreview>{/if}
          <h3>Compare commits</h3>
          <form
            class="query"
            onsubmit={(event) => {
              event.preventDefault();
              void compare();
            }}
          >
            <label
              >From commit<input
                bind:value={compareFrom}
                spellcheck="false"
              /></label
            ><button>Show diff</button>
          </form>
          {#if differences}<p class="muted">
              {differences.changes.length} changed paths
            </p>
            {#each differences.changes as change}<article class="diff">
                <h4>{change.path}</h4>
                {#if change.content.status === "text"}{#each change.content.hunks as hunk}{#if hunk.kind === "matching"}<TextPreview
                        >{hunk.before}</TextPreview
                      >{:else}<TextPreview class="removed"
                        >{hunk.before}</TextPreview
                      >
                      <TextPreview class="added">{hunk.after}</TextPreview
                      >{/if}{/each}{:else}<p>
                    Text diff unavailable: {change.content.reason.replaceAll(
                      "_",
                      " ",
                    )}
                  </p>{/if}
                <details>
                  <summary>Native terms and copy sources</summary>
                  <TextPreview>{preview(change)}</TextPreview>
                </details>
              </article>{/each}{/if}
          <h3>Conflicts</h3>
          {#if conflicts?.conflicts.length}{#each conflicts.conflicts as conflict}<article
                class="conflict"
              >
                <h4>{conflict.path}</h4>
                {#each conflict.terms as term, i}<details>
                    <summary
                      >{term.role} · {term.label ?? `Term ${term.id}`}</summary
                    >
                    <TextPreview
                      >{conflict.editor.status === "text"
                        ? conflict.editor.contents[i]
                        : preview(term.value)}</TextPreview
                    >
                  </details>{/each}{#if conflict.editor.status === "unsupported"}<p
                  >
                    Text editor unavailable: {conflict.editor.reason.replaceAll(
                      "_",
                      " ",
                    )}
                  </p>{/if}<code class="full-id">{conflict.fingerprint}</code>
              </article>{/each}{:else}<p class="muted">
              {conflicts
                ? "No unresolved conflicts."
                : "Checking conflict descriptors…"}
            </p>{/if}
        {:else if detail}
          {#if opDiff?.data.status === "select_parent"}<h3>
              Choose comparison parent
            </h3>
            {#each opDiff.data.parents as parent}<button
                onclick={() => compareOperation(parent)}>{short(parent)}</button
              >{/each}{:else if opDiff}<h3>Operation changes</h3>
            <TextPreview>{preview(opDiff.data)}</TextPreview>{/if}
        {:else if !detailBusy}<p class="empty">
            Choose a commit or operation to inspect its native data, files and
            relationships.
          </p>{/if}
        {#if detail}<details class="native">
            <summary>Native metadata</summary>
            <TextPreview>{preview(detail)}</TextPreview>
          </details>{/if}
      </section>
    </div>
  {/if}
  {#if error}<div class="error" role="alert">
      <strong>Request needs attention</strong>
      <p>{error}</p>
      {#if neededIndex}<p>
          Prepare the index for operation <code
            >{short(neededIndex.operation_id)}</code
          >.
        </p>
        <button
          onclick={prepare}
          disabled={task?.status === "running" || task?.status === "canceling"}
          >Prepare index</button
        >{/if}{#if task}<p>Index task: {task.status}</p>
        {#if task.status === "running"}<button onclick={cancelTask}
            >Cancel preparation</button
          >{/if}{/if}<button class="text-button" onclick={() => (error = "")}
        >Dismiss</button
      >
    </div>{/if}
</main>
<footer>
  <span>jj-store</span><span>Native history · explicit snapshots</span>
</footer>
