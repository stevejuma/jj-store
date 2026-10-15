<script lang="ts">
  import FilePenLine from "@lucide/svelte/icons/file-pen-line";
  import CircleCheck from "@lucide/svelte/icons/circle-check";
  import { forgeMenuLayout } from "./menu-layout.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import PictureInPicture2 from "@lucide/svelte/icons/picture-in-picture-2";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import StoreDetachablePanel from "../store-panels/StoreDetachablePanel.svelte";
  import StoreForgeDescription from "./StoreForgeDescription.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  import "./forge-inline.css";
  let { controller, branch }: { controller: ForgeController; branch: string } =
    $props();
  const id = $props.id();
  const blocked = $derived(
    controller.busy ||
      Boolean(controller.pending) ||
      (controller.accounts.managed && !controller.ready),
  );
  const plan = $derived(controller.plan);
  const item = $derived(
    plan?.items.find((item) => item.segment.branch === branch),
  );
</script>

<StoreDetachablePanel title={`Create PR · ${branch}`}>
  {#snippet children({ floating, toggle })}
    <section
      class="forge-create"
      aria-label={`Create PR for ${branch}`}
      aria-busy={controller.busy}
    >
      {#if controller.pending}
        <p role="status">
          The exact submission is retained until its outcome is known.
        </p>
        <Button
          variant="outline"
          disabled={controller.busy}
          onclick={() => controller.checkPending()}
          >Check submission outcome</Button
        >
      {/if}
      {#if item && plan}
        <Field.Group>
          <Field.Field>
            <Field.Label for={`${id}-title`}>PR title</Field.Label>
            <Input
              id={`${id}-title`}
              bind:value={item.segment.title}
              disabled={blocked}
              required
            />
          </Field.Field>
          <Field.Field>
            <StoreForgeDescription
              bind:value={item.segment.body}
              editing
              disabled={blocked}
              label={`PR description for ${branch}`}
            >
              {#snippet additionalActions()}
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={floating
                    ? "Dock PR composer"
                    : "Float PR composer"}
                  title={floating ? "Dock PR composer" : "Float PR composer"}
                  onclick={toggle}
                >
                  <PictureInPicture2 aria-hidden="true" />
                </Button>
              {/snippet}
            </StoreForgeDescription>
          </Field.Field>
        </Field.Group>
        {#if plan.items.length > 1}
          <details>
            <summary>Stack dependencies ({plan.items.length - 1})</summary>
            {#each plan.items.filter((entry) => entry !== item) as dependency}
              <Field.Group>
                <p>{dependency.segment.branch} → {dependency.segment.base}</p>
                <Field.Field
                  ><Field.Label for={`${id}-${dependency.segment.branch}`}
                    >PR title for {dependency.segment.branch}</Field.Label
                  >
                  <Input
                    id={`${id}-${dependency.segment.branch}`}
                    bind:value={dependency.segment.title}
                    disabled={blocked}
                  /></Field.Field
                >
                <StoreForgeDescription
                  bind:value={dependency.segment.body}
                  editing
                  disabled={blocked}
                  label={`PR description for ${dependency.segment.branch}`}
                />
              </Field.Group>
            {/each}
          </details>
        {/if}
        <p class="forge-create__target">{branch} → {item.segment.base}</p>
        <div class="forge-create__footer">
          <Button
            variant="outline"
            disabled={blocked}
            aria-label="Cancel PR creation"
            onclick={() => controller.inline.cancel()}>Cancel</Button
          >
          <Button
            disabled={blocked ||
              plan.items.some((entry) => !entry.segment.title.trim())}
            onclick={() => controller.submit()}>Create PR</Button
          >
          <Menu.Root
            ><Menu.Trigger
              >{#snippet child({ props })}
                <Button
                  {...props}
                  variant="outline"
                  size="sm"
                  disabled={blocked}
                  aria-label="Publication status"
                >
                  {plan.draft ? "Draft PR" : "Ready for review"}<ChevronDown
                    data-icon="inline-end"
                    aria-hidden="true"
                  />
                </Button>{/snippet}</Menu.Trigger
            >
            <Menu.Content
              align="end"
              preventScroll={false}
              style={forgeMenuLayout}
              ><Menu.Group
                ><Menu.Label>Publication status</Menu.Label>
                <Menu.RadioGroup
                  value={plan.draft ? "draft" : "ready"}
                  onValueChange={(value) => {
                    if (plan) plan.draft = value === "draft";
                  }}
                >
                  <Menu.RadioItem value="draft"
                    ><FilePenLine aria-hidden="true" />Draft PR</Menu.RadioItem
                  ><Menu.RadioItem value="ready"
                    ><CircleCheck aria-hidden="true" />Ready for review</Menu.RadioItem
                  >
                </Menu.RadioGroup>
              </Menu.Group></Menu.Content
            >
          </Menu.Root>
        </div>
      {:else}
        {#if controller.busy}<div
            role="status"
            aria-label="Preparing pull request"
          >
            <Skeleton class="forge-create__skeleton" />
          </div>{:else}<Button
            variant="outline"
            onclick={() => controller.inline.start(branch)}
            >Retry PR preview</Button
          >{/if}
        <Button
          variant="outline"
          disabled={blocked}
          aria-label="Cancel PR creation"
          onclick={() => controller.inline.cancel()}>Cancel</Button
        >
      {/if}
    </section>
  {/snippet}
</StoreDetachablePanel>
