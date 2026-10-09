<script lang="ts">
  import { untrack } from "svelte";
  import StoreAgentPromptContent from "../../web/src/lib/ui/store-agent-execution/StoreAgentPromptContent.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    empty = false,
    wide = false,
    initialDelivery = "queued",
  }: { empty?: boolean; wide?: boolean; initialDelivery?: string } = $props();
  let delivery = $state(untrack(() => initialDelivery));
  const wire = $derived(
    empty
      ? ""
      : `<agent_context>\n\n<origin>\n\n## Captured task\n\nKeep **Markdown**, [safe links](https://example.com) and code.\n\n\`\`\`xml\n<literal />\n\`\`\`\n\n&lt;instructions>captured text&lt;/instructions>\n\n</origin>\n\n</agent_context>`,
  );
</script>

<main style:max-width={wide ? "none" : "48rem"} style:padding="1rem">
  <h1>Context delivery</h1>
  <Button onclick={() => (delivery = "sent")}>Acknowledge start</Button>
  <article aria-label="User message">
    <StoreAgentPromptContent
      content="Please update the parser."
      context={wire}
      sentContext={{
        version: 1,
        delivery,
        wireContext: wire,
        wireText: "Please update the parser.",
        blocks: [],
      }}
    />
  </article>
</main>
