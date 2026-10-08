<script lang="ts">
  import BotIcon from "@lucide/svelte/icons/bot";
  import MousePointer2Icon from "@lucide/svelte/icons/mouse-pointer-2";
  import SparklesIcon from "@lucide/svelte/icons/sparkles";
  import TerminalIcon from "@lucide/svelte/icons/terminal";

  let {
    runtimeAgent,
    class: className = "",
  }: { runtimeAgent: string; class?: string } = $props();

  const Icon = $derived(
    runtimeAgent === "codex"
      ? SparklesIcon
      : runtimeAgent === "cursor"
        ? MousePointer2Icon
        : runtimeAgent === "opencode"
          ? TerminalIcon
          : BotIcon,
  );
</script>

{#if runtimeAgent === "local"}<img
    src="/agent-icons/jj-store.svg"
    class={`agent-editor__jj-agent-icon ${className}`}
    alt=""
    aria-hidden="true"
  />{:else}<Icon class={className} aria-hidden="true" />{/if}

<style>
  .agent-editor__jj-agent-icon {
    width: 1em;
    height: 1em;
    object-fit: contain;
  }
</style>
