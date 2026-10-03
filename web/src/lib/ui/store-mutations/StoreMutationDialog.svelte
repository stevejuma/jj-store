<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import {
    ConfirmButton,
    DialogFrame,
  } from "@lapismd/design-core/forms/dialog";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreMutationFields from "./StoreMutationFields.svelte";
  import StoreMutationPreview from "./StoreMutationPreview.svelte";
  import {
    buildMutationCommand,
    mutationActionCopy,
    type StoreMutationAction,
    type StoreMutationFormValue,
  } from "./model.js";
  import "./store-mutations.css";

  let {
    controller,
    action,
    open,
    capturedHead,
  }: {
    controller: StoreController;
    action: StoreMutationAction;
    open: boolean;
    capturedHead?: import("../../api/client.js").Head;
  } = $props();
  const view = $derived(controller.state);
  const mutation = $derived(view.mutation);
  const copy = $derived(mutationActionCopy[action]);
  const pending = $derived(
    mutation.status === "previewing" || mutation.status === "applying",
  );
  const revisions = $derived(
    view.graph.revisions.filter((revision) => revision.changeId.length > 0),
  );
  let form = $state<StoreMutationFormValue>({
    description: "",
    workspace: "",
    bookmark: "",
    target: "",
    base: "",
    expectedTargets: [],
    retainBookmarks: false,
    restoreDescendants: true,
  });
  let validation = $state("");
  let initialized = false;

  $effect(() => {
    if (open && !initialized) {
      if (!(
        capturedHead &&
        mutation.command?.type === "reconcile_operations" &&
        ["unknown", "applying"].includes(mutation.status)
      ))
        initialize();
      initialized = true;
    } else if (!open && initialized) {
      controller.actions.resetMutation();
      validation = "";
      initialized = false;
    }
  });

  function initialize() {
    const selected =
      view.route.commitId ??
      view.targets[0] ??
      view.graph.revisions[0]?.eventId ??
      "";
    const bookmark = view.bookmarks.find(
      (item) => item.name === view.route.bookmark,
    );
    const revision = view.graph.revisions.find(
      (item) => item.eventId === selected,
    );
    Object.assign(form, {
      description: action === "describe" ? (revision?.message ?? "") : "",
      workspace: "",
      bookmark: bookmark?.name ?? view.route.bookmark,
      target: selected,
      base:
        capturedHead?.heads[0] ??
        view.route.snapshotOperationId ??
        view.operationHeads[0] ??
        "",
      expectedTargets: [...(bookmark?.targets ?? [])],
      retainBookmarks: false,
      restoreDescendants: true,
    });
    controller.actions.resetMutation();
  }

  function changed() {
    validation = "";
    if (mutation.status !== "idle") controller.actions.resetMutation();
  }

  async function preview() {
    validation = "";
    try {
      await controller.actions.previewMutation(
        buildMutationCommand(action, form),
        capturedHead,
      );
    } catch (cause) {
      validation = cause instanceof Error ? cause.message : "Invalid command.";
    }
  }
</script>

<DialogFrame
  title={copy.title}
  description={copy.description}
  size="form"
  busy={pending}
  preventScroll={false}
>
  {#if mutation.status === "idle" || (mutation.status === "failed" && !mutation.result)}
    <form
      id="store-mutation-form"
      onsubmit={(event) => {
        event.preventDefault();
        void preview();
      }}
    >
      <StoreMutationFields
        {action}
        operationHeads={capturedHead?.heads}
        bind:description={form.description}
        bind:workspace={form.workspace}
        bind:bookmark={form.bookmark}
        bind:target={form.target}
        bind:base={form.base}
        bind:retainBookmarks={form.retainBookmarks}
        bind:restoreDescendants={form.restoreDescendants}
        selectedRevision={revisions.find(
          (item) => item.eventId === form.target,
        )}
        {revisions}
        now={view.now}
        onChange={changed}
      />
    </form>
    {#if validation || mutation.error}<p
        class="store-mutation-error"
        role="alert"
      >
        {validation || mutation.error}
      </p>{/if}
  {:else if mutation.result}
    <StoreMutationPreview result={mutation.result} />
    {#if mutation.status === "unknown"}<Alert.Root>
        <Alert.Title>Publication outcome unknown</Alert.Title>
        <Alert.Description>
          Keep this dialog open and check the durable request status. Retrying
          uses the same request ID.
        </Alert.Description>
      </Alert.Root>
    {:else if mutation.status === "committed"}<Alert.Root>
        <Alert.Title>Repository updated</Alert.Title>
        <Alert.Description>
          The committed receipt was selected and the displayed snapshot was
          refreshed.
        </Alert.Description>
      </Alert.Root>
    {:else if mutation.status === "failed"}<Alert.Root variant="destructive">
        <Alert.Title>Command failed</Alert.Title>
        <Alert.Description>{mutation.error}</Alert.Description>
      </Alert.Root>{/if}
  {/if}
  {#snippet footer()}
    {#if mutation.status === "idle" || (mutation.status === "failed" && !mutation.result)}
      <Dialog.Close>
        {#snippet child({ props })}<Button
            {...props}
            type="button"
            variant="outline">Cancel</Button
          >{/snippet}
      </Dialog.Close>
      <Button type="submit" form="store-mutation-form" disabled={pending}>
        {#if pending}<Spinner data-icon="inline-start" />{/if}{copy.preview}
      </Button>
    {:else if mutation.status === "prepared"}
      <Button
        type="button"
        variant="outline"
        onclick={controller.actions.resetMutation}>Back</Button
      >
      {#if copy.destructive}<ConfirmButton
          title={`${copy.title}?`}
          description="Apply the exact prepared native command. This publishes a new repository operation."
          confirmLabel="Apply command"
          destructive
          onConfirm={controller.actions.applyMutation}
          >Apply command</ConfirmButton
        >{:else}<Button type="button" onclick={controller.actions.applyMutation}
          >Apply command</Button
        >{/if}
    {:else if mutation.status === "unknown"}
      <Button
        type="button"
        variant="outline"
        onclick={controller.actions.applyMutation}>Retry request</Button
      >
      <Button type="button" onclick={controller.actions.checkMutationStatus}
        >Check status</Button
      >
    {:else if mutation.status === "failed" && mutation.result}
      <Button
        type="button"
        variant="outline"
        onclick={controller.actions.resetMutation}>Back</Button
      >
      <Button type="button" onclick={controller.actions.applyMutation}
        >Retry request</Button
      >
    {:else if mutation.status === "committed"}
      <Dialog.Close>
        {#snippet child({ props })}<Button {...props} type="button"
            >Close</Button
          >{/snippet}
      </Dialog.Close>
    {:else}
      <Button type="button" disabled
        ><Spinner data-icon="inline-start" />Working</Button
      >
    {/if}
  {/snippet}
</DialogFrame>
