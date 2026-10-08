<script lang="ts">
  import StoreAgentCapturedOrigin from "./StoreAgentCapturedOrigin.svelte";
  import { pretty } from "../../api/client.js";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import { object, text } from "../../controller/agent-execution-types.js";
  let { value, fallback = "" }: { value?: unknown; fallback?: string } =
    $props();
  const context = $derived(object(value));
  const blocks = $derived(
    Array.isArray(context.blocks) ? context.blocks.map(object) : [],
  );
  const wire = $derived(
    typeof context.wireContext === "string" ? context.wireContext : fallback,
  );
  const present = $derived(
    wire.trim().length > 0 ||
      (typeof context.wireContext !== "string" &&
        blocks.some(
          (b) => text(b.text).trim() || Object.keys(object(b.data)).length > 0,
        )),
  );
  const label = $derived(
    context.delivery === "sent"
      ? "Sent context"
      : context.delivery === "queued"
        ? "Queued context"
        : context.delivery === "failed"
          ? "Context not sent"
          : context.delivery === "unknown"
            ? "Context delivery unknown"
            : "Prepared context",
  );
</script>

{#if present}
  <details class="sent-context" data-ui-component="agent-sent-context">
    <summary>{label}</summary>
    <section aria-label="Application context for this message">
      {#if wire}<CommunityMarkdownContent content={wire} promptElements />
      {:else}
        {#each blocks as block, index (index)}
          <h3>{text(block.title)}</h3>
          {@const data = object(block.data)}
          {#if data.origin || data.repositories}
            {@const origin = object(data.origin)}
            {#if origin.context}<StoreAgentCapturedOrigin
                value={text(origin.context)}
              />{/if}
            {#if Array.isArray(data.repositories)}
              {#each data.repositories.map(object) as repository, index (index)}
                <dl>
                  <dt>Repository</dt>
                  <dd>{text(repository.repository)}</dd>
                  <dt>Starting revision</dt>
                  <dd>{text(repository.base_commit ?? repository.commit)}</dd>
                  <dt>Operation</dt>
                  <dd>
                    {text(
                      repository.selected_operation ?? repository.operation,
                    )}
                  </dd>
                </dl>
              {/each}
            {/if}
          {:else}<CommunityMarkdownContent content={text(block.text)} />{/if}
        {/each}
      {/if}
      <details>
        <summary>Raw context payload</summary>
        <pre>{pretty(value ?? { wireContext: fallback })}</pre>
      </details>
    </section>
  </details>
{/if}

<style>
  .sent-context {
    margin-block: 0.5rem;
    font-size: var(--text-sm, 0.875rem);
  }
  summary {
    cursor: pointer;
    width: fit-content;
    padding: 0.25rem 0.6rem;
    border: 1px solid var(--border);
    border-radius: 1rem;
  }
  section {
    margin-top: 0.5rem;
    padding: 0.75rem;
    border-radius: 0.75rem;
    background: var(--muted);
    max-height: 24rem;
    overflow: auto;
  }
  h3 {
    font-size: inherit;
    font-weight: 600;
    margin-block: 0.75rem 0.25rem;
  }
  pre,
  dd {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  pre {
    font-size: 0.75rem;
  }
  dl {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: 0.25rem 0.75rem;
  }
</style>
