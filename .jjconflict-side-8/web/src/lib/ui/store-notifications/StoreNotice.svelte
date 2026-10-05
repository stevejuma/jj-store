<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import type {
    NotificationInput,
    NotificationLifetime,
  } from "../../notifications/notification-service.svelte.js";
  import StoreNotificationHost from "./StoreNotificationHost.svelte";
  let {
    noticeKey,
    title,
    message,
    severity = "error",
    lifetime = "condition",
    pending = false,
    action,
    scope,
  }: {
    noticeKey: string;
    title: string;
    message: string | null | undefined;
    severity?: NotificationInput["severity"];
    lifetime?: NotificationLifetime;
    pending?: boolean;
    action?: NotificationInput["action"];
    scope?: string;
  } = $props();
  const { context, owned } = ownNotifications();
  let last:
    | {
        scope: string;
        key: string;
        lifetime: NotificationLifetime;
        message: string;
      }
    | undefined;
  onDestroy(() => {
    if (last?.lifetime === "condition")
      context.service.withdraw(last.scope, last.key);
  });
  $effect(() => {
    const input = {
      scope: scope ?? context.scope,
      key: noticeKey,
      title,
      message: message ?? "",
      severity,
      lifetime,
      action,
    };
    if (pending && !input.message) return;
    untrack(() => {
      // Completed outcomes belong to the originating request, even when it
      // routes to a new snapshot before its producer unmounts.
      if (
        last?.key === input.key &&
        last.lifetime !== "condition" &&
        last.message &&
        last.message === input.message
      )
        input.scope = last.scope;
      if (
        last &&
        (last.scope !== input.scope || last.key !== input.key) &&
        last.lifetime === "condition"
      )
        context.service.withdraw(last.scope, last.key);
      last = input;
      context.service.publish(input);
    });
  });
</script>

{#if owned}<StoreNotificationHost service={context.service} />{/if}
