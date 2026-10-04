<script lang="ts">
  import * as Breadcrumb from "@lapismd/design-core/shadcn/breadcrumb";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import type { StoreController } from "../../controller/contracts.js";
  import { repositoryBreadcrumbSegments } from "./repository-browser.js";

  let {
    controller,
    editing,
    filename = $bindable(""),
  }: {
    controller: StoreController;
    editing: boolean;
    filename?: string;
  } = $props();

  const view = $derived(controller.state);
  const segments = $derived(
    repositoryBreadcrumbSegments(view.route.path ?? view.route.directory),
  );
  const contextSegments = $derived(editing ? segments.slice(0, -1) : segments);
</script>

<nav class="store-application__body-breadcrumb">
  <Breadcrumb.Root>
    <Breadcrumb.List aria-label="Repository breadcrumb">
      <Breadcrumb.Item>
        <Breadcrumb.Link
          href="#"
          onclick={(event) => {
            event.preventDefault();
            controller.actions.openRepository(view.route.repository);
          }}
          ><span>{view.repository.name || view.route.repository}</span
          ></Breadcrumb.Link
        >
      </Breadcrumb.Item>
      <Breadcrumb.Separator />
      <Breadcrumb.Item>
        {#if segments.length > 0}
          <Breadcrumb.Link
            href="#"
            onclick={(event) => {
              event.preventDefault();
              controller.actions.selectDirectory("");
            }}><span>Files</span></Breadcrumb.Link
          >
        {:else}
          <Breadcrumb.Page><span>Files</span></Breadcrumb.Page>
        {/if}
      </Breadcrumb.Item>
      {#each contextSegments as segment, index (segment.path)}
        <Breadcrumb.Separator />
        <Breadcrumb.Item>
          {#if !editing && index === contextSegments.length - 1}
            <Breadcrumb.Page><span>{segment.name}</span></Breadcrumb.Page>
          {:else}
            <Breadcrumb.Link
              href="#"
              onclick={(event) => {
                event.preventDefault();
                controller.actions.selectDirectory(segment.path);
              }}><span>{segment.name}</span></Breadcrumb.Link
            >
          {/if}
        </Breadcrumb.Item>
      {/each}
      {#if editing}
        <Breadcrumb.Separator />
        <Breadcrumb.Item class="store-application__breadcrumb-filename">
          <Input
            aria-label="File name"
            title="File name relative to the repository root"
            bind:value={filename}
            spellcheck="false"
            autocomplete="off"
          />
        </Breadcrumb.Item>
      {/if}
    </Breadcrumb.List>
  </Breadcrumb.Root>
</nav>
