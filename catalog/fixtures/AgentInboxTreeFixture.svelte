<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import CommunityAppShell from "../../web/src/lib/ui/community-app-shell/CommunityAppShell.svelte";
  import CommunityInbox from "../../web/src/lib/ui/community-inbox/CommunityInbox.svelte";
  import { createStoreShell } from "../../web/src/lib/ui/store-application/create-store-shell.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    repositoryName = "Sample repository",
    loading = false,
    chatTitle = "First chat",
    threadTitle = "Selected chat",
  }: {
    repositoryName?: string;
    loading?: boolean;
    chatTitle?: string;
    threadTitle?: string;
  } = $props();
  const shell = createStoreShell(createMemoryStoryStorage(), false);
  let active = $state("first"),
    draft = $state("Retained conversation draft"),
    refreshed = $state(false),
    sessionsVisible = $state(true),
    sessionsLoaded = $state(false);
  const loadingList = $derived(loading && !sessionsLoaded);
  const author = (name: string) => ({
    id: name,
    name,
    initials: "AG",
    handle: "",
    avatar: "🤖",
  });
  const repository = (id: string) => ({
    id,
    name: refreshed && id === "repo-a" ? "Renamed repository" : repositoryName,
    label: "Open Sample repository",
    href: `#/repos/${id}/files?at_op=original-${id}`,
  });
  const items = $derived([
    {
      id: "first",
      author: author(chatTitle),
      context: "Codex",
      status: "Offline",
      repository: repository("repo-a"),
      lastActivityAt: 1_800_000_000,
    },
    {
      id: "second",
      author: author("Second chat"),
      context: "Cursor",
      status: "Online",
      repository: {
        ...repository("repo-a"),
        href: "#/repos/repo-a/files?at_op=other-operation&workspace=retained-workspace",
      },
    },
    {
      id: "other",
      author: author("Other repository chat"),
      context: "Codex",
      status: "Offline",
      repository: repository("repo-b"),
    },
    {
      id: "draft",
      author: author("New chat"),
      context: "Choose a repository",
      status: "Draft",
    },
  ]);
</script>

<CommunityAppShell
  controller={shell}
  displayMode="desktop"
  style="height: 100dvh"
>
  <AppShell.Main>
    <CommunityInbox
      items={!loadingList && sessionsVisible
        ? refreshed
          ? [...items].reverse()
          : items
        : []}
      loading={loadingList}
      activeId={active}
      layoutStorage={createMemoryStoryStorage()}
      onSelect={(id) => (active = id)}
      onBack={() => {}}
      onNew={() => (active = "draft")}
      {threadTitle}
    >
      {#snippet threadActions()}<Button
          variant="ghost"
          size="icon-sm"
          aria-label="Conversation options"
          ><EllipsisIcon aria-hidden="true" /></Button
        >{/snippet}
      <div class="fixture-content">
        {#if loadingList}<Button onclick={() => (sessionsLoaded = true)}
            >Finish loading sessions</Button
          >{/if}
        <p>Selected session: {active}</p>
        <Input aria-label="Conversation draft" bind:value={draft} />
        <Button onclick={() => (refreshed = !refreshed)}
          >Refresh and rename repository</Button
        >
        <Button onclick={() => (sessionsVisible = !sessionsVisible)}
          >{sessionsVisible
            ? "Hide loaded sessions"
            : "Restore loaded sessions"}</Button
        >
        <Button onclick={() => (active = "other")}
          >Select other repository externally</Button
        >
        <Button onclick={() => (active = "first")}
          >Select first chat externally</Button
        >
      </div>
    </CommunityInbox>
  </AppShell.Main>
</CommunityAppShell>

<style>
  .fixture-content {
    display: grid;
    gap: 1rem;
    padding: 1rem;
  }
</style>
