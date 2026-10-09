<script lang="ts">
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import CodeXmlIcon from "@lucide/svelte/icons/code-xml";
  import StoreAgentCapturedOrigin from "./StoreAgentCapturedOrigin.svelte";
  import { pretty } from "../../api/client.js";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import { object, text } from "../../controller/agent-execution-types.js";
  let { value, fallback = "" }: { value?: unknown; fallback?: string } =
    $props();
  let open = $state(false);
  let raw = $state(false);
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
  <div class="sent-context" data-ui-component="agent-sent-context">
    <Collapsible.Root bind:open>
      <div class="sent-context-heading">
        <Collapsible.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              size="xs"
              class="sent-context-trigger"
            >
              <span>{label}</span>
              <ChevronRightIcon data-icon="inline-end" aria-hidden="true" />
            </Button>
          {/snippet}
        </Collapsible.Trigger>
        {#if open}
          <span class="sent-context-toggle" class:active={raw}>
            <Button
              variant="ghost"
              size="icon-xs"
              aria-label={raw ? "Show rendered context" : "Show raw context"}
              title={raw ? "Show rendered context" : "Show raw context"}
              aria-pressed={raw}
              onclick={() => (raw = !raw)}
              ><CodeXmlIcon aria-hidden="true" /></Button
            >
          </span>
        {/if}
      </div>
      <Collapsible.Content forceMount>
        {#snippet child({ props })}
          <section
            {...props}
            hidden={!open}
            aria-label="Application context for this message"
          >
            <div hidden={raw}>
              {#if wire}<CommunityMarkdownContent
                  content={wire}
                  promptElements
                />
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
                          <dd>
                            {text(repository.base_commit ?? repository.commit)}
                          </dd>
                          <dt>Operation</dt>
                          <dd>
                            {text(
                              repository.selected_operation ??
                                repository.operation,
                            )}
                          </dd>
                        </dl>
                      {/each}
                    {/if}
                  {:else}<CommunityMarkdownContent
                      content={text(block.text)}
                    />{/if}
                {/each}
              {/if}
            </div>
            {#if raw}
              <div role="region" aria-label="Raw context">
                <CodeBlock
                  code={pretty(value ?? { wireContext: fallback })}
                  language="json"
                  size="sm"
                  width="100%"
                  container="section"
                  hasLanguageLabel={false}
                  isWrapped
                />
              </div>
            {/if}
          </section>
        {/snippet}
      </Collapsible.Content>
    </Collapsible.Root>
  </div>
{/if}

<style>
  .sent-context {
    margin-block: 0.5rem;
    font-size: var(--text-sm, 0.875rem);
  }
  .sent-context-heading {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
  :global(.sent-context-trigger[aria-expanded="true"] > svg) {
    rotate: 90deg;
  }
  .sent-context-toggle {
    opacity: 0;
  }
  .sent-context-heading:hover .sent-context-toggle,
  .sent-context-heading:focus-within .sent-context-toggle,
  .sent-context-toggle.active {
    opacity: 1;
  }
  @media (hover: none) {
    .sent-context-toggle {
      opacity: 1;
    }
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
  dd {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  dl {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: 0.25rem 0.75rem;
  }
</style>
