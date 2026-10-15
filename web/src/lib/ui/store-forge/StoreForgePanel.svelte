<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import X from "@lucide/svelte/icons/x";
  import GitPullRequest from "@lucide/svelte/icons/git-pull-request";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  import StoreForgeSetup from "./StoreForgeSetup.svelte";
  import StoreForgePullRequest from "./StoreForgePullRequest.svelte";
  import StoreForgeDescription from "./StoreForgeDescription.svelte";
  import StoreForgeChecks from "./StoreForgeChecks.svelte";
  import "./store-forge.css";
  let { controller }: { controller: ForgeController } = $props();
  const notifications = ownNotifications();
  onMount(() => {
    controller.notifications = notifications.context;
  });
  const blocked = $derived(controller.busy || Boolean(controller.pending));
</script>

<section
  class="forge-panel"
  aria-label="Pull requests"
  aria-busy={controller.busy}
>
  <ColumnCanvas.Header>
    <GitPullRequest size={16} aria-hidden="true" />
    <ColumnCanvas.Title>Pull requests</ColumnCanvas.Title>
    <ColumnCanvas.HeaderActions>
      <Tooltip.Provider>
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
      <div class="forge-panel__toolbar">
        <span>{controller.data.connection.repository}</span><Button
          variant="outline"
          size="sm"
          disabled={blocked}
          onclick={() => controller.refresh()}>Refresh PRs</Button
        >
      </div>
      <label
        >Session token<Input
          type="password"
          autocomplete="off"
          bind:value={controller.token}
        /></label
      >
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
      <StoreForgePullRequest {controller} />
      <StoreForgeChecks {controller} />
      {#if controller.logs}<pre
          aria-label="Job logs">{controller.logs}</pre>{/if}
      <nav aria-label="Repository pull requests">
        {#each controller.data.pull_requests as pr}<Button
            variant="ghost"
            disabled={blocked}
            onclick={() => controller.select(pr.head)}
            >#{pr.number} {pr.title} · {pr.state}</Button
          >{/each}
      </nav>
    {/if}
  </ScrollArea>
</section>
