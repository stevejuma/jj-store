<script lang="ts">
  import * as Chat from "@lapismd/design-core/ai/chat";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import EyeIcon from "@lucide/svelte/icons/eye";
  import TranscriptToolRichDetail from "./TranscriptToolRichDetail.svelte";
  import {
    richTranscriptToolDetail,
    type TranscriptToolEntry,
  } from "./transcript-tool-rich.js";
  import { capturedToolError } from "./transcript-tool-adapters.js";
  let {
    tool,
    value,
    raw = $bindable(false),
  }: { tool: TranscriptToolEntry; value: unknown; raw?: boolean } = $props();
  const rich = $derived(
    richTranscriptToolDetail({ ...tool, error: undefined }),
  );
  const error = $derived(capturedToolError(tool));
  const facts = $derived(
    value && typeof value === "object"
      ? (value as Record<string, unknown>)
      : {},
  );
</script>

<div
  class="transcript-tool-detail"
  style="--ui-code-syntax-variable:var(--foreground);"
  data-mode={raw ? "raw" : "rich"}
  data-ui-component="agent-tool-detail"
>
  <div class="transcript-tool-detail__controls">
    <Button
      variant="ghost"
      size="sm"
      aria-label={raw ? "Show rich output" : "Show raw output"}
      aria-pressed={raw}
      onclick={() => (raw = !raw)}
    >
      {#if raw}<EyeIcon data-icon="inline-start" />Formatted{:else}<EyeIcon
          data-icon="inline-start"
        />Raw JSON{/if}
    </Button>
  </div>
  {#if facts.redacted || facts.truncated || tool.redacted || tool.truncated}<p
      class="transcript-tool-detail__notice"
    >
      {facts.redacted || tool.redacted
        ? "Sensitive fields were redacted. "
        : ""}{facts.truncated || tool.truncated ? "Output was truncated." : ""}
    </p>{/if}
  {#if raw}
    <CodeBlock
      code={JSON.stringify(value, null, 2)}
      language="json"
      title="Tool activity"
      size="sm"
      width="full"
      isWrapped
      maxHeight="20rem"
    />
  {:else}
    {#if error}<p
        class="transcript-tool-detail__error"
        data-ui-part="tool-error"
      >
        {error}
      </p>{/if}
    {#if rich}<TranscriptToolRichDetail
        detail={rich}
      />{:else}<Chat.ToolCallDetail
        input={tool.input}
        output={tool.output}
        toolName={tool.name}
      />{/if}
    {#if !rich && !tool.input && !tool.output && !error}<p
        class="transcript-tool-detail__notice"
      >
        No displayable output was captured. Raw JSON retains the recorded
        fields.
      </p>{/if}
  {/if}
</div>

<style>
  .transcript-tool-detail {
    display: grid;
    gap: 0.5rem;
    min-width: 0;
  }
  .transcript-tool-detail__controls {
    display: flex;
    justify-content: flex-end;
  }
  .transcript-tool-detail__notice,
  .transcript-tool-detail__error {
    margin: 0;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  .transcript-tool-detail__error {
    max-height: 20rem;
    overflow: auto;
    color: var(--foreground);
  }
</style>
