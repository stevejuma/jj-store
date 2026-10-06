<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let { manager }: { manager: AgentExecution } = $props();
  const formId = $props.id();
  let targetId = $state("");
  let runner = $state("codex");
  let setup = $state("");
  let commit = $state("");
  const target = $derived(manager.targets.find((t) => t.id === targetId));
  const saved = $derived(manager.value?.setups.find((s) => s.id === setup));
  $effect(() => {
    if (!targetId && manager.targets.length) targetId = manager.targets[0].id;
  });
  $effect(() => {
    const selected = targetId,
      contextual = manager.context.commit;
    untrack(() => {
      const target = manager.targets.find((t) => t.id === selected);
      if (target) commit = contextual || target.commit;
    });
  });
  $effect(() => {
    if (saved) runner = saved.runner;
  });
  const valid = $derived(/^(?:[0-9a-f]{40}|[0-9a-f]{128})$/.test(commit));
</script>

<form
  class="agent-execution-form"
  aria-label="Start agent session"
  onsubmit={(event) => {
    event.preventDefault();
    if (target && valid) void manager.launch(target, runner, setup, commit);
  }}
>
  <h3>Start a session</h3>
  {#if manager.value?.runtime === "preparing"}<p role="status">
      Preparing agent sessions…
    </p>{:else if manager.value?.runtime === "unavailable"}<p role="status">
      Agent execution is unavailable on this device.
    </p>{/if}
  <p>
    Create an isolated JJ workspace from the selected revision. Uncommitted
    source-folder edits stay in the source folder.
  </p>
  {#if !manager.targets.length}<p role="status">
      Register a local JJ folder or link a native repository workspace to launch
      a session.
    </p>{/if}
  <label for={`${formId}-target`}>Repository workspace</label>
  <select id={`${formId}-target`} bind:value={targetId} disabled={manager.busy}
    >{#each manager.targets as item (item.id)}<option value={item.id}
        >{item.name} · {item.profile} · {item.folder}</option
      >{/each}</select
  >
  <label for={`${formId}-revision`}>Exact starting revision</label>
  <Input
    id={`${formId}-revision`}
    bind:value={commit}
    required
    aria-invalid={!valid}
    disabled={manager.busy}
  />
  {#if commit && !valid}<p>
      Use the full revision ID from this repository.
    </p>{/if}
  {#if target}<p class="agent-execution-meta">
      Operation: {target.operation}
    </p>{/if}
  <label for={`${formId}-setup`}>Saved setup</label>
  <select id={`${formId}-setup`} bind:value={setup} disabled={manager.busy}
    ><option value="">No saved setup</option
    >{#each manager.value?.setups ?? [] as item (item.id)}<option
        value={item.id}>{item.avatar} {item.name}</option
      >{/each}</select
  >
  <label for={`${formId}-runner`}>Runner</label>
  <select
    id={`${formId}-runner`}
    bind:value={runner}
    disabled={manager.busy || !!saved}
    >{#each manager.value?.runners ?? [] as item (item.id)}<option
        value={item.id}
        >{item.label}{item.available ? "" : " · unavailable"}</option
      >{/each}</select
  >
  {#if manager.context.text}<label for={`${formId}-context`}
      >Context for the first prompt</label
    ><Textarea
      id={`${formId}-context`}
      bind:value={manager.context.text}
      rows={4}
    />{/if}
  <Button
    type="submit"
    disabled={manager.busy ||
      !!manager.pending ||
      !target ||
      !valid ||
      !manager.value?.runners.find((r) => r.id === runner)?.available ||
      manager.value?.runtime !== "ready"}>Create session workspace</Button
  >
</form>
