<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { untrack } from "svelte";
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { provideSettingsPresentation } from "../store-settings/settings-presentation.js";
  import History from "@lucide/svelte/icons/history";
  import GitCommit from "@lucide/svelte/icons/git-commit-horizontal";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import { createWorkflowRunSource } from "./workflow-run-source.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StoreDataSource } from "../../controller/native-data-source.js";
  import "../store-settings/StoreSettings.css";
  import "../store-settings/creation-forms.css";
  import type { WorkflowVersion } from "../../controller/workflow-types.js";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  let {
    workflow,
    work,
    acquireRepository,
    repositoryName,
    onClose,
  }: {
    workflow: WorkflowVersion;
    work: WorkflowController;
    acquireRepository?: StoreController["acquireRepository"];
    repositoryName?: string;
    onClose(): void;
  } = $props();
  const id = $props.id();
  provideSettingsPresentation({ value: "global" });
  const retained = untrack(() =>
    work.state.pending?.command.type === "run" &&
    work.state.pending.command.workflow === workflow.id
      ? work.state.pending.command.source
      : null,
  );
  let operation = $state(retained?.operation ?? ""),
    commit = $state(retained?.commit ?? ""),
    bookmark = $state(retained?.bookmark ?? ""),
    reader = $state<StoreDataSource>(),
    connectionError = $state(""),
    reconnect = $state(0);
  const disabled = $derived(work.state.writing || !!work.state.pending);
  const choices = createWorkflowRunSource(
    () => reader,
    () => operation,
  );
  $effect(() => {
    const repository = workflow.definition.repository,
      acquire = acquireRepository,
      attempt = reconnect;
    let lease: StoreDataSource | undefined;
    untrack(() => {
      connectionError = "";
      try {
        if (!acquire)
          throw Error(
            "Source revision choices are unavailable for this connection.",
          );
        lease = acquire(repository);
        reader = lease;
      } catch (cause) {
        reader = undefined;
        connectionError =
          cause instanceof Error
            ? cause.message
            : "Source repository unavailable.";
      }
    });
    return () => lease?.close();
  });
  const valid = $derived(
    /^[0-9a-f]{128}$/.test(operation) && /^[0-9a-f]{128}$/.test(commit),
  );
  async function run() {
    if (
      await work.submit({
        type: "run",
        workflow: workflow.id,
        expected: workflow.version,
        source: {
          repository: workflow.definition.repository,
          operation,
          commit,
          bookmark: bookmark || null,
        },
      })
    )
      onClose();
  }
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open && !work.state.writing) onClose();
  }}
  ><DialogFrame
    size="form"
    title={`Run ${workflow.definition.name}`}
    description="Choose the exact recorded source revision."
    busy={work.state.writing}
  >
    <div class="store-creation-form store-settings--global">
      <StoreSettingsCard
        id={`${id}-source`}
        title="Source revision"
        description={`Choose recorded code from ${repositoryName || workflow.definition.repository.slice(0, 12)}.`}
        Icon={GitBranch}
      >
        <Field.Group class="store-settings__editable-rows">
          <StoreWorkField
            label="Operation"
            id={`${id}-operation`}
            description="Choose the repository state to run against."
            Icon={History}
            class="store-settings__option-row"
            {disabled}
          >
            <StoreRepositoryPicker
              id={`${id}-operation`}
              label="Operation"
              repositories={choices.operations.rows}
              value={operation}
              controlled
              emptyLabel="Choose operation"
              groupLabel="Operations"
              searchLabel="Find an operation"
              emptyResult="No matching loaded operations."
              moreLabel="operations"
              unavailableLabel="Selected operation"
              hasMore={!!choices.operations.cursor}
              loading={choices.operations.loading}
              notice={choices.operations.loading
                ? "Loading operations…"
                : undefined}
              disabled={disabled || !reader}
              onLoadMore={() => void choices.loadOperations(true)}
              onChange={(value) => {
                operation = value;
                commit = "";
                bookmark = "";
              }}
            >
              {#snippet triggerIcon()}<History aria-hidden="true" />{/snippet}
              {#snippet itemIcon()}<History aria-hidden="true" />{/snippet}
            </StoreRepositoryPicker>
          </StoreWorkField>
          <StoreWorkField
            label="Commit"
            id={`${id}-commit`}
            description="Select a commit recorded at this operation."
            Icon={GitCommit}
            class="store-settings__option-row"
            {disabled}
          >
            <StoreRepositoryPicker
              id={`${id}-commit`}
              label="Commit"
              repositories={choices.commits.rows}
              value={commit}
              controlled
              emptyLabel="Choose commit"
              groupLabel="Commits"
              searchLabel="Find a commit"
              emptyResult="No matching loaded commits."
              moreLabel="commits"
              unavailableLabel="Selected commit"
              hasMore={!!choices.commits.cursor}
              loading={choices.commits.loading}
              notice={choices.commits.loading ? "Loading commits…" : undefined}
              disabled={disabled || !reader || !operation}
              onLoadMore={() => void choices.loadRevisions("commits", true)}
              onChange={(value) => {
                commit = value;
                bookmark = "";
              }}
            >
              {#snippet triggerIcon()}<GitCommit aria-hidden="true" />{/snippet}
              {#snippet itemIcon()}<GitCommit aria-hidden="true" />{/snippet}
            </StoreRepositoryPicker>
          </StoreWorkField>
          <StoreWorkField
            label="Bookmark (optional)"
            id={`${id}-bookmark`}
            description="Choose a bookmark to select its exact commit."
            Icon={GitBranch}
            class="store-settings__option-row"
            {disabled}
          >
            <StoreRepositoryPicker
              id={`${id}-bookmark`}
              label="Bookmark (optional)"
              repositories={choices.bookmarks.rows}
              value={bookmark}
              controlled
              allowEmpty
              emptyLabel="No bookmark"
              groupLabel="Bookmarks"
              searchLabel="Find a bookmark"
              emptyResult="No matching loaded bookmarks."
              moreLabel="bookmarks"
              unavailableLabel="Selected bookmark"
              hasMore={!!choices.bookmarks.cursor}
              loading={choices.bookmarks.loading}
              notice={choices.bookmarks.loading
                ? "Loading bookmarks…"
                : undefined}
              disabled={disabled || !reader || !operation}
              onLoadMore={() => void choices.loadRevisions("bookmarks", true)}
              onChange={(value) => {
                bookmark = value;
                if (value)
                  commit =
                    choices.bookmarks.rows.find((row) => row.id === value)
                      ?.commit ?? "";
              }}
            >
              {#snippet triggerIcon()}<GitBranch aria-hidden="true" />{/snippet}
              {#snippet itemIcon()}<GitBranch aria-hidden="true" />{/snippet}
            </StoreRepositoryPicker>
          </StoreWorkField>
        </Field.Group>
      </StoreSettingsCard>
    </div>
    <StoreNotice
      noticeKey={`workflow-run-connection-${id}`}
      title="Source repository unavailable"
      message={connectionError}
      action={{
        id: "retry-source",
        label: "Retry source",
        run: () => {
          reconnect++;
        },
      }}
    />
    <StoreNotice
      noticeKey={`workflow-run-operations-${id}`}
      title="Operations could not be loaded"
      message={choices.operations.error}
      pending={choices.operations.loading}
      action={{
        id: "retry-operations",
        label: "Retry operations",
        run: () => void choices.loadOperations(),
      }}
    />
    {#each ["commits", "bookmarks"] as kind (kind)}{@const key = kind as
        "commits" | "bookmarks"}
      <StoreNotice
        noticeKey={`workflow-run-${key}-${id}-${operation}`}
        title={`${key === "commits" ? "Commits" : "Bookmarks"} could not be loaded`}
        message={choices[key].error}
        pending={choices[key].loading}
        action={{
          id: `retry-${key}`,
          label: `Retry ${key}`,
          run: () => void choices.loadRevisions(key),
        }}
      />
    {/each}
    {#snippet footer()}<Button
        variant="outline"
        disabled={work.state.writing}
        onclick={onClose}>Cancel</Button
      ><Button
        disabled={!valid || work.state.writing || !!work.state.pending}
        onclick={run}>Run workflow</Button
      >{/snippet}
  </DialogFrame></Dialog.Root
>
