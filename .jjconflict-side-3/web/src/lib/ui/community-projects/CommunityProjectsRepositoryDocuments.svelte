<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import BookOpenIcon from "@lucide/svelte/icons/book-open";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import HeartHandshakeIcon from "@lucide/svelte/icons/heart-handshake";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import ScaleIcon from "@lucide/svelte/icons/scale";
  import UsersIcon from "@lucide/svelte/icons/users";
  import { onMount, tick } from "svelte";

  import type { RepositoryBlob } from "../../controller/contracts.js";
  import { repositoryLandingDocumentLabel } from "./repository-browser.js";
  import CommunityProjectsMarkdownPreview from "./CommunityProjectsMarkdownPreview.svelte";
  import "./repository-documents.css";

  let {
    currentPath,
    previewDocuments,
    onEditFile,
    editUnavailable = "",
  }: {
    currentPath: string;
    previewDocuments: readonly RepositoryBlob[];
    onEditFile?: ((path: string) => void) | undefined;
    editUnavailable?: string | undefined;
  } = $props();

  const landingDocuments = $derived(
    currentPath
      ? []
      : previewDocuments
          .map((document) => ({
            ...document,
            label: repositoryLandingDocumentLabel(document.path),
          }))
          .filter(
            (
              document,
            ): document is RepositoryBlob & {
              label: string;
              contents: string;
            } =>
              Boolean(document.label) &&
              !document.binary &&
              !document.too_large &&
              document.contents !== undefined,
          )
          .sort(
            (left, right) =>
              [
                "README",
                "Code of conduct",
                "Contributing",
                "License",
                "Security",
              ].indexOf(left.label) -
                [
                  "README",
                  "Code of conduct",
                  "Contributing",
                  "License",
                  "Security",
                ].indexOf(right.label) || left.path.localeCompare(right.path),
          ),
  );
  let selectedDocument = $state("");
  let visibleDocumentCount = $state(5);
  let documentHeader: HTMLElement | null = $state(null);
  let documentTabList: HTMLElement | null = $state(null);
  let measuring = false;
  const visibleDocuments = $derived(
    landingDocuments.slice(0, visibleDocumentCount),
  );
  const overflowDocuments = $derived(
    landingDocuments.slice(visibleDocumentCount),
  );
  const overflowSelectionActive = $derived(
    overflowDocuments.some((document) => document.path === selectedDocument),
  );
  const selectedDocumentModel = $derived(
    landingDocuments.find((document) => document.path === selectedDocument),
  );

  $effect(() => {
    const first = landingDocuments[0]?.path ?? "";
    if (
      first &&
      !landingDocuments.some((document) => document.path === selectedDocument)
    ) {
      selectedDocument = first;
    } else if (!first) {
      selectedDocument = "";
    }
  });

  async function fitTabs() {
    if (measuring || !documentHeader || !documentTabList) return;
    const header = documentHeader;
    const tabs = documentTabList;
    measuring = true;
    visibleDocumentCount = landingDocuments.length;
    await tick();
    while (
      documentHeader === header &&
      documentTabList === tabs &&
      tabs.isConnected &&
      visibleDocumentCount > 1 &&
      tabs.scrollWidth > tabs.clientWidth + 1
    ) {
      visibleDocumentCount -= 1;
      await tick();
    }
    measuring = false;
  }

  $effect(() => {
    landingDocuments.length;
    void tick().then(fitTabs);
  });

  onMount(() => {
    if (!documentHeader) return;
    const observer = new ResizeObserver(() => void fitTabs());
    observer.observe(documentHeader);
    void fitTabs();
    return () => observer.disconnect();
  });
</script>

{#if landingDocuments.length > 0}
  <section
    class="community-projects__repository-document-card"
    aria-label="Repository document preview"
  >
    <Tabs.Root
      bind:value={selectedDocument}
      class="community-projects__repository-document-tabs"
    >
      <header
        bind:this={documentHeader}
        class="community-projects__repository-document-header"
      >
        <Tabs.List
          bind:ref={documentTabList}
          variant="line"
          aria-label="Repository documents"
        >
          {#each visibleDocuments as document (document.path)}
            <Tabs.Trigger value={document.path}>
              {#if document.label === "README"}
                <BookOpenIcon data-icon="inline-start" aria-hidden="true" />
              {:else if document.label === "Code of conduct"}
                <HeartHandshakeIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />
              {:else if document.label === "Contributing"}
                <UsersIcon data-icon="inline-start" aria-hidden="true" />
              {:else}
                <ScaleIcon data-icon="inline-start" aria-hidden="true" />
              {/if}
              {document.label}
              <span
                class="community-projects__repository-document-tab-indicator"
                aria-hidden="true"
              ></span>
            </Tabs.Trigger>
          {/each}
        </Tabs.List>
        {#if overflowDocuments.length > 0}<DropdownMenu.Root>
            <DropdownMenu.Trigger>
              {#snippet child({ props })}<Button
                  {...props}
                  type="button"
                  variant="ghost"
                  size="sm"
                  class="community-projects__repository-document-more"
                  data-selected={overflowSelectionActive ? "" : undefined}
                  aria-label="More repository documents"
                  >More<ChevronDownIcon
                    data-icon="inline-end"
                    aria-hidden="true"
                  /><span
                    class="community-projects__repository-document-tab-indicator"
                    aria-hidden="true"
                  ></span></Button
                >{/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
              align="start"
              sideOffset={4}
              preventScroll={false}
              aria-label="More repository documents"
            >
              {#each overflowDocuments as document (document.path)}
                <DropdownMenu.Item
                  class="community-projects__repository-document-menu-item"
                  data-selected={document.path === selectedDocument
                    ? ""
                    : undefined}
                  onSelect={() => (selectedDocument = document.path)}
                >
                  {document.label}
                </DropdownMenu.Item>
              {/each}
            </DropdownMenu.Content>
          </DropdownMenu.Root>{/if}
        {#if selectedDocumentModel}<Button
            type="button"
            variant="ghost"
            size="icon-sm"
            class="community-projects__repository-document-edit"
            aria-label={`Edit ${selectedDocumentModel.label}`}
            title={editUnavailable || `Edit ${selectedDocumentModel.label}`}
            disabled={Boolean(editUnavailable) || !onEditFile}
            onclick={() => onEditFile?.(selectedDocumentModel.path)}
          >
            <PencilIcon aria-hidden="true" />
          </Button>{/if}
      </header>
      {#each landingDocuments as document (document.path)}
        <Tabs.Content value={document.path}>
          <CommunityProjectsMarkdownPreview
            content={document.contents}
            label={`${document.label} preview`}
          />
        </Tabs.Content>
      {/each}
    </Tabs.Root>
  </section>
{/if}
