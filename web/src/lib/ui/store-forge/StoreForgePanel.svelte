<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import Pencil from "@lucide/svelte/icons/pencil";
  import X from "@lucide/svelte/icons/x";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import StoreForgeIcon from "./StoreForgeIcon.svelte";
  import StoreForgePanelActions from "./StoreForgePanelActions.svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  import StoreForgeSetup from "./StoreForgeSetup.svelte";
  import StoreForgePullRequest from "./StoreForgePullRequest.svelte";
  import StoreForgePullRequestActions from "./StoreForgePullRequestActions.svelte";
  import StoreForgeDescription from "./StoreForgeDescription.svelte";
  import "./store-forge.css";
  let { controller }: { controller: ForgeController } = $props();
  const notifications = ownNotifications();
  onMount(() => {
    controller.notifications = notifications.context;
  });
  let editing = $state(false);
  const blocked = $derived(controller.busy || Boolean(controller.pending));
</script>

<section
  class="forge-panel"
  aria-label="Pull requests"
  aria-busy={controller.busy}
>
  <ColumnCanvas.Header>
    <StoreForgeIcon provider={controller.data?.connection?.provider} />
    <ColumnCanvas.Title role="heading" aria-level={2}
      >{controller.selected
        ? controller.title || controller.selected.title
        : "Pull requests"}</ColumnCanvas.Title
    >
    {#if controller.selected}
      <a
        class="forge-pr__number"
        href={controller.selected.url}
        target="_blank"
        rel="noreferrer">#{controller.selected.number}</a
      >
      <CommunityChip variant="muted"
        >{controller.selected.draft
          ? "Draft"
          : controller.selected.state}</CommunityChip
      >
    {/if}
    <ColumnCanvas.HeaderActions class="forge-header-actions">
      {#if controller.selected}<StoreForgePullRequestActions
          {controller}
        />{/if}
      <Tooltip.Provider>
        {#if controller.selected && controller.data}
          <Tooltip.Root>
            <Tooltip.Trigger>
              {#snippet child({ props })}
                <Button
                  {...props}
                  variant="ghost"
                  size="icon-sm"
                  aria-label="Edit details"
                  aria-pressed={editing}
                  disabled={blocked || !controller.data?.capabilities.edit}
                  onclick={() => (editing = !editing)}
                  ><Pencil aria-hidden="true" /></Button
                >
              {/snippet}
            </Tooltip.Trigger>
            <Tooltip.Content>Edit details</Tooltip.Content>
          </Tooltip.Root>
        {/if}
        <StoreForgePanelActions {controller} />
        <Tooltip.Root>
          <Tooltip.Trigger>
            {#snippet child({ props })}
              <Button
                {...props}
                variant="ghost"
                size="icon-sm"
                aria-label="Close pull requests"
                onclick={() => controller.closePanel()}
              >
                <X aria-hidden="true" />
              </Button>
            {/snippet}
          </Tooltip.Trigger>
          <Tooltip.Content>Close pull requests</Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
    </ColumnCanvas.HeaderActions>
  </ColumnCanvas.Header>
  <ScrollArea scrollStrategy="styled" class="forge-panel__body">
    {#if !controller.data?.connection}
      <StoreForgeSetup {controller} />
    {:else}
      {#if controller.stale}<p role="status">
          Remote information needs refresh.
        </p>{/if}
      {#if controller.pending}<p role="status">
          The exact action is retained while its outcome is checked.
        </p>{/if}
      {#if controller.data.receipt}
        <section aria-label="Submission progress">
          <h3>Submission {controller.data.receipt.outcome}</h3>
          <p>Request {controller.data.receipt.plan.request_id}</p>
          <ol>
            {#each controller.data.receipt.steps as step}<li>
                {step.branch}: {step.kind} · {step.outcome}
              </li>{/each}
          </ol>
          {#if controller.data.receipt.outcome !== "complete"}<Button
              variant="outline"
              disabled={controller.busy}
              onclick={() => controller.recover()}
              >Check original submission</Button
            >{/if}
        </section>
      {/if}
      {#if controller.restack}
        <section aria-label="Restack preview">
          <h3>Restack remaining changes</h3>
          <p>
            Rebase {controller.restack.plan.branch} and its descendants onto {controller
              .restack.plan.connection.target}@{controller.restack.plan
              .connection.target_remote}, then push and update the remaining
            PRs.
          </p>
          <p>Landed: {controller.restack.plan.merged.join(", ") || "none"}</p>
          <p>Target: {controller.restack.plan.target_head}</p>
          <Button disabled={blocked} onclick={() => controller.applyRestack()}
            >Rebase and publish stack</Button
          >
        </section>
      {/if}
      {#if controller.plan}
        <section aria-label="Stack submission preview">
          <h3>Submit stack</h3>
          {#if controller.plan.cumulative}<p>
              Fork stack: PRs target upstream with cumulative diffs. Merge
              bottom first.
            </p>{/if}
          <label
            ><input
              type="checkbox"
              checked={!controller.plan.draft}
              onchange={(event) => {
                if (controller.plan)
                  controller.plan.draft = !event.currentTarget.checked;
              }}
            /> Ready for review</label
          >
          {#each controller.plan.items as item}<fieldset>
              <legend>{item.segment.branch} → {item.segment.base}</legend>
              <p>{item.steps.join(", ")}</p>
              <label>PR title<Input bind:value={item.segment.title} /></label>
              <StoreForgeDescription
                bind:value={item.segment.body}
                editing
                label={`PR description for ${item.segment.branch}`}
                disabled={blocked}
              />
            </fieldset>{/each}
          <Button disabled={blocked} onclick={() => controller.submit()}
            >Push branches and submit</Button
          >
        </section>
      {/if}
      <StoreForgePullRequest {controller} bind:editing />
      {#if !controller.selected && !controller.plan}<nav
          aria-label="Repository pull requests"
        >
          {#each controller.data.pull_requests as pr}<Button
              variant="ghost"
              disabled={blocked}
              onclick={() => controller.select(pr.head)}
              >#{pr.number} {pr.title} · {pr.state}</Button
            >{/each}
        </nav>{/if}
    {/if}
  </ScrollArea>
</section>
