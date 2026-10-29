<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { ownNotifications } from "../../web/src/lib/notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../../web/src/lib/ui/store-notifications/StoreNotificationHost.svelte";
  import StoreNotice from "../../web/src/lib/ui/store-notifications/StoreNotice.svelte";
  const { context } = ownNotifications();
  let error = $state("");
  let visible = $state(true);
  let dialog = $state(false);
  let draft = $state("Keep this draft");
  function fail() {
    error = "The request failed. Your draft is retained.";
  }
</script>

<Button onclick={fail}>Fail request</Button>
<Button onclick={() => (error = "")}>Recover</Button>
<Button onclick={() => (visible = !visible)}>Toggle page</Button>
<FixtureControls floating={false}
  ><Button
    onclick={() =>
      context.service.publish({
        scope: "application",
        key: "acknowledge",
        title: "Attention required",
        message: "Review the completed operation.",
        severity: "warning",
        lifetime: "dismiss",
      })}>Show sticky warning</Button
  ></FixtureControls
>
<Button
  onclick={() =>
    context.service.publish({
      scope: "application",
      key: "saved",
      title: "Saved",
      message: "Settings saved.",
      severity: "info",
      lifetime: "timed",
    })}>Show confirmation</Button
>
<Button onclick={() => (dialog = true)}>Edit</Button>
{#if visible}<StoreNotice
    noticeKey="request"
    title="Request failed"
    message={error}
  />{/if}
<Dialog.Root bind:open={dialog}>
  <Dialog.Content>
    <Dialog.Header
      ><Dialog.Title>Edit details</Dialog.Title><Dialog.Description
        >Your draft survives request errors.</Dialog.Description
      ></Dialog.Header
    >
    <label>Draft<input bind:value={draft} /></label>
    <Button onclick={fail}>Save draft</Button>
    <Button onclick={() => (dialog = false)}>Close editor</Button>
  </Dialog.Content>
</Dialog.Root>
<StoreNotificationHost service={context.service} />
