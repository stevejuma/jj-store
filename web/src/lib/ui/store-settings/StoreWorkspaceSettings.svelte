<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
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
  }: {
    settings: WorkspaceLinkSettings;
    supported: boolean;
    onRefresh: () => Promise<void>;
    onPath: (path: string) => void;
    onLink: () => Promise<void>;
  } = $props();
  const busy = $derived(
    settings.workspaceLinking ||
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
        onclick={() => void onRefresh()}>Refresh folders</Button
      >{/if}
  </header>
  {#if !supported}
    <p class="store-settings__empty">
      Local folder management is not available for this connection.
    </p>
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
          <Field.Description id="workspace-folder-help"
            >Enter the absolute path of an existing native JJ workspace
            connected to this local service.</Field.Description
          >
        </Field.Field>
        <Field.Field orientation="horizontal">
          <Button
            type="submit"
            disabled={busy ||
              !settings.workspaceLinks ||
              !settings.workspacePath}>Link folder</Button
          >
          <Button
            type="button"
            variant="outline"
            disabled={busy || !settings.workspacePath}
            onclick={() => onPath("")}>Cancel</Button
          >
        </Field.Field>
      </Field.Group>
    </form>
    {#if settings.workspaceLinksError}
      <Alert.Root variant="destructive"
        ><Alert.Title>Local folders unavailable</Alert.Title><Alert.Description
          >{settings.workspaceLinksError}</Alert.Description
        ></Alert.Root
      >
    {/if}
  {/if}
</section>
