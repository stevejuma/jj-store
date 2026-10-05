<script lang="ts">
  import { useAppearance } from "../../appearance/store.svelte.js";
  import { useDiscussionFocus } from "./discussion-focus.svelte.js";
  import StoreDiscussionSheet from "./StoreDiscussionSheet.svelte";
  const appearance = useAppearance();
  const owner = useDiscussionFocus();
  let { open = true }: { open?: boolean } = $props();
  const active = $derived(owner?.active);
</script>

<StoreDiscussionSheet
  open={open && appearance?.live.threads === "focus" && !!active}
  title={active?.title() ?? "Discussion"}
  returnFocus={active?.returnFocus}
  onClose={() => active?.close()}
>
  {#if active}
    {@const discussion = active.content()}
    {@render discussion()}
  {/if}
</StoreDiscussionSheet>
