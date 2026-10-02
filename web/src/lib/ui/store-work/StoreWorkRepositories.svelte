<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "./StoreWorkField.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import StoreWorkStatus from "./StoreWorkStatus.svelte";
  import "./community-work-rows.css";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import StoreRepositoryDirectoryRow from "./StoreRepositoryDirectoryRow.svelte";
  import ArchiveIcon from "@lucide/svelte/icons/archive";
  import UndoIcon from "@lucide/svelte/icons/undo-2";
  import LinkIcon from "@lucide/svelte/icons/link";
  import XIcon from "@lucide/svelte/icons/x";
  import type { RepositoryDirectoryEntry } from "../../controller/projects-repositories.svelte.js";
  import type { Model } from "../../api/client.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import type { WorkController } from "../../controller/work-types.js";
  let {
    project,
    work,
    repositories,
    writable,
    onOpenRepository,
    initialManual = false,
    busy = false,
    layout = "list",
    query = "",
    entries = [],
    home,
    open = $bindable(false),
  }: {
    project: Model<"ProjectV1">;
    work: WorkController;
    repositories: readonly StoreRepositorySummary[];
    writable: boolean;
    initialManual?: boolean;
    busy?: boolean;
    layout?: "grid" | "list";
    query?: string;
    entries?: readonly RepositoryDirectoryEntry[];
    home?: string;
    open?: boolean;
    onOpenRepository: (id: string) => void | Promise<void>;
  } = $props();
  const formId = $props.id();
  import { untrack } from "svelte";
  let link = $state(""),
    manual = $state(untrack(() => initialManual));
  let submitted = $state<string | null>(null);
  const disabled = $derived(
    !writable ||
      busy ||
      work.state.loading ||
      work.state.writing ||
      !!work.state.pending,
  );
  async function set(repository: string, active: boolean) {
    await work.submit({
      command: "set_repository",
      project: project.id,
      expected_version: project.version,
      repository,
      active,
    });
  }
  async function submit(event: SubmitEvent) {
    event.preventDefault();
    if (disabled || !link) return;
    submitted = link;
    await set(link, true);
  }
  $effect(() => {
    // Receipt checks/retries may complete after the initial submit returns.
    if (
      submitted &&
      project.repositories[submitted] === true &&
      !work.state.pending &&
      !work.state.writing &&
      !work.state.error
    ) {
      open = false;
      link = "";
      submitted = null;
    }
  });
</script>

<section aria-label="Linked repositories">
  <div
    class="community-projects__list"
    data-layout={layout}
    role="list"
    aria-label="Repository links"
  >
    {#each Object.entries(project.repositories).filter( ([id]) => `${repositories.find((repo) => repo.id === id)?.name ?? ""} ${id}`
          .toLocaleLowerCase()
          .includes(query.toLocaleLowerCase()) ) as [id, active]}{@const repo =
        repositories.find((r) => r.id === id)}
      <div role="listitem">
        <StoreRepositoryDirectoryRow
          repository={repo ?? { id, name: id, permission: null }}
          {entries}
          project={home ? { home, id: project.id } : undefined}
          accessible={Boolean(repo)}
          status={active ? "" : "Retired"}
          onOpen={onOpenRepository}
        >
          {#snippet actions()}{#if writable}<Button
                variant="ghost"
                size="icon-sm"
                aria-label={active ? "Retire link" : "Restore link"}
                title={active ? "Retire link" : "Restore link"}
                {disabled}
                onclick={() => set(id, !active)}
                >{#if active}<ArchiveIcon aria-hidden="true" />{:else}<UndoIcon
                    aria-hidden="true"
                  />{/if}</Button
              >{/if}{/snippet}
        </StoreRepositoryDirectoryRow>
      </div>{/each}
  </div>
  {#if !Object.keys(project.repositories).length}<StoreWorkEmpty
      kind="repositories"
      title="No repositories linked yet"
      description="Link an existing repository to organize its tasks and reviews in this project."
    />{/if}
</section>
{#if writable}<Dialog.Root bind:open>
    <DialogFrame
      size="wide"
      preventScroll={false}
      busy={work.state.writing}
      title="Link repository"
      description="Link an existing repository to this project. Code access is authorized separately from project metadata."
    >
      <form id={`${formId}-form`} class="store-work__form" onsubmit={submit}>
        <Field.Group
          class="store-work__fields community-settings__option-group"
        >
          <StoreWorkField
            label="Link by repository ID"
            id={`${formId}-manual`}
            description="Enter a full identity instead of choosing an available repository."
            {disabled}
          >
            <Switch id={`${formId}-manual`} bind:checked={manual} {disabled} />
          </StoreWorkField>
          {#if manual}<StoreWorkField
              label="Repository ID"
              id={`${formId}-id`}
              description="Use the complete native repository identity."
              {disabled}
            >
              <Input
                id={`${formId}-id`}
                bind:value={link}
                pattern={"[0-9a-f]{32}"}
                required
                {disabled}
              />
            </StoreWorkField>
          {:else}<StoreWorkField
              label="Repository"
              id={`${formId}-repository`}
              description="Code access is authorized separately from project metadata."
              {disabled}
            >
              <StoreRepositoryPicker
                id={`${formId}-repository`}
                label="Repository"
                bind:value={link}
                {disabled}
                repositories={repositories.filter(
                  (r) => project.repositories[r.id] !== true,
                )}
              />
            </StoreWorkField>{/if}
        </Field.Group>
      </form>
      <StoreWorkStatus {work} />
      {#snippet footer()}
        <Button
          type="button"
          variant="outline"
          disabled={work.state.writing}
          onclick={() => (open = false)}
        >
          <XIcon data-icon="inline-start" aria-hidden="true" />Cancel
        </Button>
        <Button
          type="submit"
          form={`${formId}-form`}
          disabled={disabled || !link}
        >
          <LinkIcon data-icon="inline-start" aria-hidden="true" />Link
          repository
        </Button>
      {/snippet}
    </DialogFrame>
  </Dialog.Root>{/if}
