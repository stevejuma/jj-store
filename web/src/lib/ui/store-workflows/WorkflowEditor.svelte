<script lang="ts">
  import { untrack } from "svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import WorkflowYaml from "./WorkflowYaml.svelte";
  import WorkflowEditorMenu from "./WorkflowEditorMenu.svelte";
  import WorkflowName from "./WorkflowName.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import WorkflowForm from "./WorkflowForm.svelte";
  import {
    yamlDefinition,
    readDefinition,
    type WorkflowDraft,
  } from "./workflow-presentation.js";
  import type {
    StoreRepositorySummary,
    StoreController,
  } from "../../controller/contracts.js";
  import type {
    WorkflowDefinition,
    WorkflowVersion,
  } from "../../controller/workflow-types.js";
  let {
    initial,
    home = $bindable(),
    homes,
    repositories,
    authors = [],
    acquireRepository,
    controller,
    version = null,
    busy = false,
    pending = "",
    writable = true,
    onSave,
    onClose,
    onRetry,
    onTrigger,
    onDuplicate,
    onDelete,
  }: {
    initial: WorkflowDraft;
    home: string;
    homes: readonly StoreRepositorySummary[];
    repositories: readonly StoreRepositorySummary[];
    authors?: readonly { id: string; name: string }[];
    acquireRepository?: StoreController["acquireRepository"];
    controller?: StoreController;
    version?: WorkflowVersion | null;
    busy?: boolean;
    pending?: string;
    writable?: boolean;
    onSave(definition: WorkflowDefinition): Promise<boolean>;
    onClose(draft: WorkflowDraft): void;
    onRetry?(): void;
    onTrigger?(): void;
    onDuplicate?(draft: WorkflowDraft): void;
    onDelete?(): Promise<boolean>;
  } = $props();
  const id = $props.id();
  const { context } = ownNotifications();
  function failure(message: string, kind = "definition") {
    const key = `workflow-editor-${id}-${kind}`;
    context.service.resolve(context.scope, key);
    context.service.publish({
      scope: context.scope,
      key,
      title: "Workflow needs attention",
      message,
      severity: "error",
      lifetime: "dismiss",
    });
  }
  let definition = $state(
      untrack(() => structuredClone($state.snapshot(initial.definition))),
    ),
    mode = $state(untrack(() => initial.mode)),
    yaml = $state(untrack(() => initial.yaml)),
    unapplied = $state(untrack(() => initial.unapplied)),
    invalidInputs = $state(false),
    inspector = $state(true);
  function switchMode(value: string) {
    if (value === "form" && mode === "yaml") {
      try {
        definition = readDefinition(yaml, true);
        unapplied = false;
        context.service.resolve(
          context.scope,
          `workflow-editor-${id}-definition`,
        );
      } catch (cause) {
        unapplied = true;
        failure(
          `${cause instanceof Error ? cause.message : "Invalid YAML"}. Form shows the last valid draft; your YAML text is retained.`,
        );
      }
    } else if (value === "yaml" && !unapplied)
      yaml = yamlDefinition(definition);
    mode = value;
  }
  function close(next = definition) {
    onClose({ definition: next, mode, yaml, unapplied });
  }
  async function save() {
    try {
      const next = readDefinition(
        mode === "yaml" ? yaml : yamlDefinition(definition),
      );
      context.service.resolve(
        context.scope,
        `workflow-editor-${id}-definition`,
      );
      if (await onSave(next)) close(next);
    } catch (cause) {
      failure(cause instanceof Error ? cause.message : "Invalid workflow");
    }
  }
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open && !busy) close();
  }}
>
  <DialogFrame
    size="wide"
    surface="inset"
    class="workflow-editor-dialog"
    bodyPadding="none"
    title={version ? "Edit workflow" : "Create workflow"}
    description="Choose a repository trigger and ordered actions."
    descriptionHidden
    {busy}
    style="width:min(72rem,calc(100vw - 2rem));height:88vh;max-height:88vh;"
  >
    {#snippet headerActions()}
      <WorkflowName
        bind:value={definition.name}
        disabled={busy || !writable || mode === "yaml"}
      />
      <WorkflowEditorMenu
        enabled={definition.enabled ?? false}
        disabled={busy || !!pending || !writable}
        toggleDisabled={mode === "yaml"}
        onEnabled={(value) => (definition.enabled = value)}
        onEdit={() => {
          switchMode("form");
          inspector = true;
        }}
        {onTrigger}
        onDuplicate={() => {
          try {
            const next =
              mode === "yaml" ? readDefinition(yaml, true) : definition;
            onDuplicate?.({ definition: next, mode, yaml, unapplied });
          } catch (cause) {
            failure(
              cause instanceof Error ? cause.message : "Invalid workflow",
            );
          }
        }}
        onDelete={onDelete
          ? async () => {
              if (await onDelete!()) close();
            }
          : undefined}
      />
    {/snippet}
    {#if mode === "form"}<WorkflowForm
        {acquireRepository}
        {controller}
        {authors}
        bind:definition
        bind:inspector
        bind:home
        {homes}
        homeDisabled={busy || !!pending || !!version}
        bind:invalid={invalidInputs}
        onInvalid={(message) => failure(message, "inputs")}
        onValid={() =>
          context.service.resolve(
            context.scope,
            `workflow-editor-${id}-inputs`,
          )}
        {repositories}
        disabled={busy || !writable}
      />{:else}<div class="workflow-editor workflow-editor--yaml">
        <div class="workflow-canvas">
          <div class="workflow-context">
            <StoreRepositoryPicker
              label="Metadata home"
              repositories={homes}
              bind:value={home}
              variant="ghost"
              disabled={busy || !!pending || !!version}
            />
          </div>
          <div class="workflow-yaml">
            <WorkflowYaml bind:value={yaml} disabled={busy || !writable} />
          </div>
        </div>
      </div>{/if}
    {#snippet footer()}<div class="workflow-editor-footer">
        <Tabs.Root value={mode} onValueChange={switchMode}
          ><Tabs.List aria-label="Workflow editor mode"
            ><Tabs.Trigger value="form">Form</Tabs.Trigger><Tabs.Trigger
              value="yaml">YAML</Tabs.Trigger
            ></Tabs.List
          ></Tabs.Root
        >
        <div>
          <Button variant="outline" disabled={busy} onclick={() => close()}
            >Cancel</Button
          ><Button
            disabled={busy ||
              !!pending ||
              !writable ||
              !home ||
              (mode === "form" && invalidInputs)}
            onclick={save}>{busy ? "Saving…" : "Save workflow"}</Button
          >
        </div>
      </div>{/snippet}
  </DialogFrame>
</Dialog.Root>
