<script lang="ts">
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import LinkIcon from "@lucide/svelte/icons/link";
  import XIcon from "@lucide/svelte/icons/x";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import type { WorkspaceLinkSettings } from "../../controller/workspace-link-types.js";
  let {
    settings,
    supported,
    onRefresh,
    onPath,
    onLink,
    pickerSupported,
    onBrowse,
  }: {
    settings: WorkspaceLinkSettings;
    supported: boolean;
    onRefresh: () => Promise<void>;
    onPath: (path: string) => void;
    onLink: () => Promise<void>;
    pickerSupported: boolean;
    onBrowse: () => Promise<void>;
  } = $props();
  const busy = $derived(
    settings.workspaceLinking ||
      settings.workspacePicking ||
      ["loading", "refreshing"].includes(settings.workspaceLinksState),
  );
</script>

<section
  class="store-settings__section"
  aria-labelledby="repository-local-folders"
>
  <header class="store-settings__section-heading">
    <div>
      <h2 id="repository-local-folders">Local folders</h2>
      <p>Physical workspaces linked to this repository on this device.</p>
    </div>
    {#if supported}<Button
        variant="outline"
        disabled={busy}
        onclick={() => void onRefresh()}
        ><RefreshCwIcon data-icon="inline-start" aria-hidden="true" />Refresh
        folders</Button
      >{/if}
  </header>
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
      <p class="store-settings__empty">No local folders linked.</p>
    {/if}
    {#if busy}<p role="status">
        {settings.workspaceLinking
          ? "Linking folder…"
          : settings.workspacePicking
            ? "Choose a folder in the system dialog…"
            : "Loading local folders…"}
      </p>{/if}
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void onLink();
      }}
    >
      <Field.Group>
        <Field.Field data-disabled={busy}>
          <Field.Label for="workspace-folder-path">Folder path</Field.Label>
          <ButtonGroup.Root
            class="store-settings__field-action"
            style="width: 100%"
            aria-label="Choose local folder"
          >
            <Input
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
              <Button
                type="button"
                variant="outline"
                disabled={busy}
                onclick={() => void onBrowse()}
                ><GitBranchIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />Browse…</Button
              >
            {/if}
          </ButtonGroup.Root>
          <Field.Description id="workspace-folder-help"
            >Enter the absolute path of an existing native JJ workspace
            connected to this local service. {#if pickerSupported}Browse opens a
              dialog on the machine running the local service.{/if}</Field.Description
          >
        </Field.Field>
        <Field.Field orientation="horizontal">
          <Button
            type="submit"
            disabled={busy ||
              !settings.workspaceLinks ||
              !settings.workspacePath}
            ><LinkIcon data-icon="inline-start" aria-hidden="true" />Link folder</Button
          >
          <Button
            type="button"
            variant="outline"
            disabled={busy || !settings.workspacePath}
            onclick={() => onPath("")}
            ><XIcon data-icon="inline-start" aria-hidden="true" />Cancel</Button
          >
        </Field.Field>
      </Field.Group>
    </form>
    {#if settings.workspacePickerError}
      <Alert.Root variant="destructive"
        ><Alert.Title>Folder picker unavailable</Alert.Title>
        <Alert.Description>{settings.workspacePickerError}</Alert.Description
        ></Alert.Root
      >
    {/if}
    {#if settings.workspaceLinksError}
      <Alert.Root variant="destructive"
        ><Alert.Title>Local folders unavailable</Alert.Title><Alert.Description
          >{settings.workspaceLinksError}</Alert.Description
        ></Alert.Root
      >
    {/if}
  {/if}
</section>
