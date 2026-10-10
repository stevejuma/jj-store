<script lang="ts">
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import "../store-agent-chat-body/text-reader.css";
  let {
    text,
    expanded,
    onExpandedChange,
  }: {
    text: string;
    expanded: boolean;
    onExpandedChange: (value: boolean) => void;
  } = $props();
  const id = $props.id();
  const preview = $derived(
    text.slice(0, 600).split("\n").slice(0, 8).join("\n"),
  );
  const truncated = $derived(preview.length < text.length);
</script>

<p {id} class="prompt-queue__text">
  {expanded || !truncated ? text : `${preview.trimEnd()}…`}
</p>
{#if truncated}
  <div class="chat-middle-fold">
    <button
      type="button"
      aria-controls={id}
      aria-expanded={expanded}
      onclick={() => onExpandedChange(!expanded)}
    >
      {#if expanded}<ChevronDownIcon
          size={16}
          aria-hidden="true"
        />{:else}<ChevronRightIcon size={16} aria-hidden="true" />{/if}
      {expanded ? "See Less" : "See More"}
    </button>
    <span></span>
  </div>
{/if}

<style>
  .prompt-queue__text {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    margin: 0;
  }
</style>
