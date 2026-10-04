<script lang="ts">
  import { untrack, type ComponentProps } from "svelte";
  import StoreSearchCommand from "../../web/src/lib/ui/store-search/StoreSearchCommand.svelte";

  let { args }: { args: ComponentProps<typeof StoreSearchCommand> } = $props();
  let open = $state(untrack(() => args.open ?? true));
  let query = $state(untrack(() => args.query));
</script>

<div data-search-open={open ? "true" : "false"}>
  <StoreSearchCommand
    {...args}
    bind:open
    {query}
    onQueryChange={(value) => {
      query = value;
      args.onQueryChange(value);
    }}
  />
</div>
