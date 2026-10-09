<script lang="ts">
  import { decode } from "../../api/client.js";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import { object, text } from "../../controller/agent-execution-types.js";
  let { value }: { value: string } = $props();
  const captured = $derived.by(() => {
    try {
      return object(decode(value));
    } catch {
      return null;
    }
  });
  const reply = $derived(object(captured?.reply));
  const file = $derived(object(captured?.file));
  const labels: Record<string, string> = {
    repository: "Repository",
    project: "Project",
    task: "Task",
    thread: "Thread",
    path: "File",
    revision: "Revision",
    operation: "Operation",
    review: "Review",
  };
</script>

{#if captured}
  <dl>
    {#each Object.entries(labels) as [key, label] (key)}
      {#if text(captured[key])}<dt>{label}</dt>
        <dd>{text(captured[key])}</dd>{/if}
    {/each}
  </dl>
  {#each ["project_content", "task_content"] as key (key)}
    {@const content = object(captured[key])}
    {#if text(content.title) || text(content.body)}
      <h4>{text(content.title)} · version {String(content.version ?? "")}</h4>
      <CommunityMarkdownContent content={text(content.body)} fullWidth />
    {/if}
  {/each}
  {#if text(file.content)}
    <h4>Captured file content{file.truncated ? " (excerpt)" : ""}</h4>
    <pre><code>{text(file.content)}</code></pre>
  {/if}
  {#if text(reply.content)}
    <h4>Reply context</h4>
    <CommunityMarkdownContent content={text(reply.content)} fullWidth />
  {/if}
{:else}
  <CommunityMarkdownContent content={value} fullWidth />
{/if}

<style>
  dl {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: 0.25rem 0.75rem;
  }
  dd,
  pre {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  h4 {
    font-size: inherit;
    font-weight: 600;
    margin-top: 0.75rem;
  }
  pre {
    font-size: 0.75rem;
    padding: 0.5rem;
    background: var(--background);
    border-radius: var(--radius);
  }
</style>
