<script lang="ts">
  import { untrack } from "svelte";
  import { useProfileDirectory } from "../../profiles/context.svelte.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { AgentSetup } from "../../controller/agent-execution-types.js";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import WorkflowSelect from "./WorkflowSelect.svelte";
  import User from "@lucide/svelte/icons/user-round";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import type { WorkflowDefinition } from "../../controller/workflow-types.js";
  let {
    id,
    definition = $bindable(),
    authors = [],
    agents = [],
    loading = false,
    hasMore = false,
    onLoadMore,
    onOpen,
    disabled = false,
    conversational = false,
  }: {
    id: string;
    definition: WorkflowDefinition;
    authors?: readonly { id: string; name: string }[];
    agents?: readonly Pick<AgentSetup, "id" | "name" | "avatar">[];
    loading?: boolean;
    hasMore?: boolean;
    onLoadMore?(): void;
    onOpen?(): void;
    disabled?: boolean;
    conversational?: boolean;
  } = $props();
  const conditions = $derived(definition.conditions ?? {});
  let mode = $state<string>();
  const excluded = $derived(
    mode ? mode === "exclude" : !!conditions.excluded_authors?.length,
  );
  let query = $state("");
  const selected = $derived(
    excluded ? (conditions.excluded_authors ?? []) : (conditions.authors ?? []),
  );
  const profileContext = useProfileDirectory();
  let profileError = $state("");
  let profileLoading = $state(false);
  const principalKey = $derived(
    conversational
      ? [...new Set([...authors.map((a) => a.id), ...selected])]
          .filter((id) => /^[0-9a-f]{32}$/.test(id))
          .sort()
          .join(",")
      : "",
  );
  let generation = 0;
  async function hydrate() {
    const directory = profileContext?.directory,
      key = principalKey,
      ticket = generation;
    if (!key || !directory?.available() || profileLoading) return;
    profileLoading = true;
    try {
      await directory.loadProfiles(key.split(","), true);
      if (ticket === generation) profileError = "";
    } catch (cause) {
      if (ticket === generation)
        profileError =
          cause instanceof Error
            ? cause.message
            : "Could not load author profiles.";
    } finally {
      if (ticket === generation) profileLoading = false;
    }
  }
  $effect(() => {
    const key = principalKey,
      directory = profileContext?.directory,
      available = directory?.available();
    untrack(() => {
      generation++;
      profileLoading = false;
      profileError = "";
      if (key && available) void hydrate();
    });
    return () => {
      generation++;
    };
  });
  const humans = $derived(
    authors.map((a) => ({
      ...a,
      name: conversational
        ? profileContext?.directory?.state.profiles[a.id]?.display_name ||
          a.name
        : a.name,
    })),
  );
  const candidates = $derived([
    ...new Map(
      [
        ...humans.filter((a) =>
          conversational ? /^[0-9a-f]{32}$/.test(a.id) : a.id.includes("@"),
        ),
        ...selected
          .filter((id) => !humans.some((a) => a.id === id))
          .map((id) => ({ id, name: id })),
        ...(!humans.some((a) => a.id === query.trim()) &&
        query.trim() &&
        (!conversational || /^[0-9a-f]{32}$/.test(query.trim()))
          ? [{ id: query.trim(), name: query.trim() }]
          : []),
      ].map((a) => [a.id, a]),
    ).values(),
  ]);
  const choices = $derived([
    ...candidates.map((a) => ({
      ...a,
      description: a.id,
      kind: "human",
      avatar: "",
      disabled: false,
    })),
    ...agents.map((a) => ({
      id: `agent:${a.id}`,
      name: a.name,
      description: "Agent message triggers are not supported yet",
      kind: "agent",
      avatar: a.avatar,
      disabled: true,
    })),
  ]);
  function choose(next: string[]) {
    definition.conditions = {
      ...conditions,
      ...(excluded ? { excluded_authors: next } : { authors: next }),
    };
  }
  function match(next: string) {
    const values = selected;
    mode = next;
    definition.conditions = {
      ...conditions,
      authors: next === "include" ? values : [],
      excluded_authors: next === "exclude" ? values : [],
    };
  }
</script>

<StoreNotice
  noticeKey={`workflow-author-profiles-${id}`}
  title="Author profiles could not be loaded"
  message={profileError}
  pending={profileLoading}
  action={{
    id: "retry-author-profiles",
    label: "Retry",
    run: () => void hydrate(),
  }}
/>
<Field.Field
  orientation="horizontal"
  class="store-settings__option-row workflow-inspector-row--wide"
>
  <Field.Content
    ><Field.Label for={`${id}-authors`}
      ><StoreSettingsIcon Icon={User} />Authors</Field.Label
    ><Field.Description
      >{conversational
        ? "Select a known author or enter a principal ID."
        : "Select a known author or enter a commit author email."}</Field.Description
    ></Field.Content
  >
  <div class="workflow-conditions__fields">
    <WorkflowSelect
      id={`${id}-author-mode`}
      label="Author match"
      value={excluded ? "exclude" : "include"}
      {disabled}
      options={[
        { value: "include", label: "Include", Icon: User },
        { value: "exclude", label: "Exclude", Icon: User },
      ]}
      onChange={match}
    />
    <StoreRepositoryPicker
      id={`${id}-authors`}
      label="Authors"
      repositories={choices}
      bind:values={() => selected, choose}
      multiple
      keepItemIcon
      selectionLabel="authors"
      allowEmpty
      limit={16}
      emptyLabel="Any author"
      searchLabel="Find or enter an author"
      groupLabel="Authors"
      emptyResult={loading
        ? "Loading authors…"
        : "No known authors. Enter an author identity."}
      notice={loading ? "Loading authors…" : undefined}
      {loading}
      {hasMore}
      moreLabel="authors"
      {onLoadMore}
      onOpen={() => {
        onOpen?.();
        void hydrate();
      }}
      {disabled}
      onSearch={(value) => (query = value)}
    >
      {#snippet triggerIcon()}<User aria-hidden="true" />{/snippet}
      {#snippet itemIcon(authorId)}
        {@const author = choices.find((a) => a.id === authorId)!}
        {#if conversational}<CommunityMemberAvatar
            member={{
              id: author.id,
              name: author.name,
              handle: author.id,
              initials: author.name.slice(0, 2).toUpperCase(),
              ...(author.kind === "agent"
                ? { kind: "agent", avatarUrl: author.avatar }
                : {}),
            }}
            size="inline"
            decorative
          />{:else}<CommunityProjectsGitAuthorAvatar
            email={author.id}
            name={author.name}
            size="xsmall"
          />{/if}
      {/snippet}
    </StoreRepositoryPicker>
  </div>
</Field.Field>

<style>
  .workflow-conditions__fields {
    display: flex;
    gap: 0.5rem;
    flex-wrap: nowrap;
    margin-left: 2.3rem;
    width: calc(100% - 2.3rem);
  }
  .workflow-conditions__fields :global([data-slot="select-trigger"]) {
    flex: 0 0 auto;
    width: auto;
  }
  .workflow-conditions__fields :global(.store-repository-picker__trigger) {
    flex: 1;
    width: 0;
  }
</style>
