<script lang="ts">
  import { settingsPresentation } from "./settings-presentation.js";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import LinkIcon from "@lucide/svelte/icons/link";
  import XIcon from "@lucide/svelte/icons/x";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import { Separator } from "@lapismd/design-core/shadcn/separator";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { WorkspaceLinkSettings } from "../../controller/workspace-link-types.js";
  let {
    settings,
    supported,
    onPath,
    onLink,
    pickerSupported,
    onBrowse,
  }: {
    settings: WorkspaceLinkSettings;
    supported: boolean;
    onPath: (path: string) => void;
    onLink: () => Promise<void>;
    pickerSupported: boolean;
    onBrowse: () => Promise<void>;
  } = $props();
  const presentation = settingsPresentation();
  const busy = $derived(
    settings.workspaceLinking ||
      settings.workspacePicking ||
      ["loading", "refreshing"].includes(settings.workspaceLinksState),
  );
</script>

<StoreSettingsCard
  id="repository-local-folders"
  title="Local folders"
  Icon={FolderGitIcon}
>
  {#if !supported}
    <StoreSettingsEmpty
      Icon={GitBranchIcon}
      title="Local folder management unavailable"
      description="Local folder management is not available for this connection."
    />
  {:else}
    {#if settings.workspaceLinks?.links.length}
      <dl class="store-settings__rows" aria-label="Linked local folders">
        {#each settings.workspaceLinks.links as link (link.id)}
          <div>
            <dt>{link.path}</dt>
            <dd>
              {link.availability === "ready" ? "Ready" : "Unavailable"}
              {#if link.availability !== "ready"}<p>
                  Check the folder’s location and repository binding.
                </p>{/if}
            </dd>
          </div>
        {/each}
      </dl>
    {:else if !busy && settings.workspaceLinks}
      <dl class="store-settings__rows">
        <div>
          <dt>
            Linked folders
            <p class="store-settings__row-description">
              Physical workspaces linked to this repository on this device.
            </p>
          </dt>
          <dd>No local folders linked.</dd>
        </div>
      </dl>
    {/if}
    {#if busy}<dl class="store-settings__rows">
        <div>
          <dt>Folder status</dt>
          <dd>
            <span role="status">
              {settings.workspaceLinking
                ? "Linking folder…"
                : settings.workspacePicking
                  ? "Choose a folder in the system dialog…"
                  : "Loading local folders…"}
            </span>
          </dd>
        </div>
      </dl>{/if}
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void onLink();
      }}
    >
      {#snippet linkActions()}
        <Button
          type="button"
          variant="outline"
          disabled={busy || !settings.workspacePath}
          onclick={() => onPath("")}
          ><XIcon data-icon="inline-start" aria-hidden="true" />Cancel</Button
        >
        <Button
          type="submit"
          disabled={busy || !settings.workspaceLinks || !settings.workspacePath}
          ><LinkIcon data-icon="inline-start" aria-hidden="true" />Link folder</Button
        >
      {/snippet}
      <div class="store-settings__rows">
        <Field.Group class="store-workspace-link-form">
          <Field.Field
            data-disabled={busy}
            orientation="horizontal"
            class="store-settings__option-row"
          >
            <Field.Content>
              <Field.Label for="workspace-folder-path">Folder path</Field.Label>
              <Field.Description id="workspace-folder-help"
                >Enter the absolute path of an existing native JJ workspace
                connected to this local service. {#if pickerSupported}Browse
                  opens a dialog on the machine running the local service.{/if}</Field.Description
              >
            </Field.Content>
            <InputGroup.Root
              class="store-settings__field-action"
              aria-label="Choose local folder"
            >
              <InputGroup.Input
                id="workspace-folder-path"
                value={settings.workspacePath}
                oninput={(event) => onPath(event.currentTarget.value)}
                required
                maxlength={4096}
                disabled={busy}
                autocomplete="off"
                spellcheck={false}
                aria-describedby="workspace-folder-help"
              />
              {#if pickerSupported}
                <InputGroup.Addon align="inline-end">
                  <InputGroup.Button
                    size="sm"
                    disabled={busy}
                    onclick={() => void onBrowse()}
                    ><GitBranchIcon
                      data-icon="inline-start"
                      aria-hidden="true"
                    />Browse…</InputGroup.Button
                  >
                </InputGroup.Addon>
              {/if}
            </InputGroup.Root>
          </Field.Field>
          {#if presentation.value === "repository"}<StoreSettingsFooter>
              {#snippet description()}<div>
                  <strong>Link an existing folder</strong>
                  <p>Linking preserves files and repository history.</p>
                </div>{/snippet}
              {@render linkActions()}
            </StoreSettingsFooter>{:else}
            <Separator />
            <footer class="store-workspace-link-form__footer">
              <div>
                <strong>Link an existing folder</strong>
                <p>Linking preserves files and repository history.</p>
              </div>
              <div class="store-workspace-link-form__actions">
                {@render linkActions()}
              </div>
            </footer>{/if}
        </Field.Group>
      </div>
      {#if settings.workspaceValidation}<p role="alert">
          {settings.workspaceValidation}
        </p>{/if}
    </form>
    <StoreNotice
      noticeKey="folder-picker"
      title="Folder picker unavailable"
      message={settings.workspacePickerError}
      pending={busy}
    />
    <StoreNotice
      noticeKey="folders"
      title="Local folders unavailable"
      message={settings.workspaceLinksError}
      pending={busy}
    />
  {/if}
</StoreSettingsCard>

<style>
  :global(.store-workspace-link-form) {
    gap: 0;
  }
  :global(.store-workspace-link-form > [data-slot="field"]) {
    display: grid;
    grid-template-columns: minmax(10rem, 0.8fr) minmax(0, 1.5fr);
    gap: 1rem;
    align-items: start;
  }
  .store-workspace-link-form__footer {
    display: grid;
    gap: 1rem;
    padding: 1rem;
  }
  .store-workspace-link-form__footer strong {
    font-size: var(--community-text-body);
    font-weight: 550;
  }
  .store-workspace-link-form__footer p {
    margin-block-start: 0.35rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
  .store-workspace-link-form__actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 0.5rem;
  }
  @media (max-width: 40rem) {
    :global(.store-workspace-link-form > [data-slot="field"]) {
      grid-template-columns: minmax(0, 1fr);
    }
  }
</style>
