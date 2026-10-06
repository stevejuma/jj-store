<script lang="ts">
  // @community-copy AgentPermissionCard.svelte (Apache-2.0); exact ACP options.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import { object, text } from "../../controller/agent-execution-types.js";
  let {
    manager,
    value,
    title,
  }: {
    manager: AgentExecution;
    value: Record<string, unknown>;
    title: string;
  } = $props();
  const request = $derived(object(value.request));
  const options = $derived(
    Array.isArray(request.options) ? request.options.map(object) : [],
  );
</script>

<article class="agent-permission-card" aria-label="Agent permission request">
  <div>
    <h3>{title || "Permission requested"}</h3>
    <p>The agent requested an explicit action.</p>
  </div>
  <CodeBlock
    code={JSON.stringify(request.toolCall ?? {}, null, 2)}
    language="json"
    title="Requested action"
    size="sm"
    width="full"
    isWrapped
    maxHeight="16rem"
  />
  <div class="agent-permission-card__actions">
    {#each options as option}<Button
        variant={text(option.kind).startsWith("reject") ? "outline" : "default"}
        size="sm"
        disabled={manager.busy ||
          !!manager.pending ||
          manager.session?.state !== "running"}
        onclick={() =>
          manager.command({
            action: "permission",
            permission_id: text(value.permissionId),
            option_id: text(option.optionId),
          })}>{text(option.name) || text(option.optionId)}</Button
      >{/each}
    <Button
      variant="ghost"
      size="sm"
      disabled={manager.busy ||
        !!manager.pending ||
        manager.session?.state !== "running"}
      onclick={() =>
        manager.command({
          action: "permission",
          permission_id: text(value.permissionId),
          option_id: null,
        })}>Cancel request</Button
    >
  </div>
</article>

<style>
  .agent-permission-card {
    display: grid;
    gap: 0.875rem;
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    background: var(--background);
  }
  h3 {
    font-size: var(--community-text-body);
    font-weight: 600;
    margin: 0;
  }
  p {
    margin: 0.25rem 0 0;
    color: var(--muted-foreground);
  }
  .agent-permission-card__actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    justify-content: flex-end;
  }
</style>
