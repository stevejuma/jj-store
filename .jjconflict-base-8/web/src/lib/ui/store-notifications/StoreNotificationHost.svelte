<script lang="ts">
  import { NotificationToasts } from "@lapismd/design-core/workspace/plugins/notifications";
  import type { NotificationService } from "../../notifications/notification-service.svelte.js";
  import type { NotificationContext } from "../../notifications/notification-context.svelte.js";
  let {
    service,
    owner,
  }: {
    service?: NotificationService;
    owner?: { owned: boolean; context: NotificationContext };
  } = $props();
  const active = $derived(
    service ?? (owner?.owned ? owner.context.service : undefined),
  );
</script>

{#if active}
  <NotificationToasts
    records={active.records}
    onDismiss={active.dismiss}
    onAction={active.invoke}
    onPause={active.pause}
    onResume={active.resume}
    portal
  />
{/if}
