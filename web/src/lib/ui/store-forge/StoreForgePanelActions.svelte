<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller }: { controller: ForgeController } = $props();
  let connectionOpen = $state(false);
  const id = $props.id();
  const blocked = $derived(
    controller.busy ||
      Boolean(controller.pending) ||
      (controller.accounts.managed && !controller.ready),
  );
</script>

{#if controller.data?.connection}
  <Menu.Root>
    <Menu.Trigger
      >{#snippet child({ props })}<Button
          {...props}
          variant="ghost"
          size="icon-sm"
          aria-label="Pull request options"
          ><Ellipsis aria-hidden="true" /></Button
        >{/snippet}</Menu.Trigger
    >
    <Menu.Content align="end" preventScroll={false}>
      <Menu.Group>
        <Menu.Label>Pull requests</Menu.Label>
        <Menu.Item disabled={blocked} onSelect={() => controller.refresh()}
          >Refresh PRs</Menu.Item
        >
        <Menu.Item
          onSelect={() =>
            controller.accounts.managed &&
            controller.data?.connection?.provider === "github"
              ? controller.accounts.openSettings?.()
              : (connectionOpen = true)}>Forge connection…</Menu.Item
        >
      </Menu.Group>
      <Menu.Separator />
      <Menu.Group
        ><Menu.Label>{controller.data.connection.repository}</Menu.Label>
        {#each controller.data.pull_requests as pr}<Menu.Item
            disabled={blocked}
            onSelect={() => controller.select(pr.head)}
            >#{pr.number} {pr.title}</Menu.Item
          >{/each}
      </Menu.Group>
    </Menu.Content>
  </Menu.Root>
  <Dialog.Root bind:open={connectionOpen}>
    <Dialog.Content>
      <Dialog.Header
        ><Dialog.Title>Forge connection</Dialog.Title><Dialog.Description
          >{controller.data.connection.repository} · {controller.data.connection
            .api_url}</Dialog.Description
        ></Dialog.Header
      >
      <Field.Group
        ><Field.Field>
          <Field.Label for={`${id}-token`}>Session token</Field.Label>
          <Input
            id={`${id}-token`}
            type="password"
            autocomplete="off"
            bind:value={controller.token}
          />
          <Field.Description>Kept in memory for this session.</Field.Description
          >
        </Field.Field></Field.Group
      >
      <Dialog.Footer
        ><Button
          disabled={blocked}
          onclick={async () => {
            await controller.refresh();
            connectionOpen = false;
          }}>Refresh PRs</Button
        ></Dialog.Footer
      >
    </Dialog.Content>
  </Dialog.Root>
{/if}
