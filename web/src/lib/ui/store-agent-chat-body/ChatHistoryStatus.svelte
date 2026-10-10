<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  let { loading = false }: { loading?: boolean } = $props();
</script>

{#if loading}<div
    class="chat-history-loading"
    role="status"
    aria-label="Loading previous messages"
  >
    <Button variant="ghost" size="xs" disabled
      ><Spinner
        data-icon="inline-start"
        role="presentation"
        aria-hidden="true"
      />Loading previous messages</Button
    >
  </div>{:else}<Empty.Root
    ><Empty.Header
      ><Empty.Media variant="icon"
        ><MessageSquareIcon aria-hidden="true" /></Empty.Media
      ><Empty.Title>No captured messages.</Empty.Title></Empty.Header
    ></Empty.Root
  >{/if}
