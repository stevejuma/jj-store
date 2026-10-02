<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import { agentMember } from "../../collaboration/presentation.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import BotIcon from "@lucide/svelte/icons/bot";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import StoreWorkCollection from "./StoreWorkCollection.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreCollaborationSkeleton from "../store-collaboration/StoreCollaborationSkeleton.svelte";
  import StoreAgentSessionRow from "../store-collaboration/StoreAgentSessionRow.svelte";
  let {
    controller,
    onSelect,
  }: { controller: StoreController; onSelect(home: string, id: string): void } =
    $props();
  const hub = $derived(controller.projects!),
    route = $derived(hub.route);
  const scopeHomes = $derived(
    route.projectId
      ? []
      : hub.navigation.homes.filter(
          (home) => !route.workHome || home.repository.id === route.workHome,
        ),
  );
  const links = $derived(
    route.projectId
      ? Object.keys(hub.detail?.state.project?.repositories ?? {})
      : [
          ...new Set(
            scopeHomes.flatMap((home) =>
              home.projects.flatMap((project) =>
                Object.keys(project.repositories),
              ),
            ),
          ),
        ],
  );
  const unavailable = $derived(
    links.filter(
      (id) => !controller.state.repositories.some((repo) => repo.id === id),
    ),
  );
  const query = $derived((route.workQuery ?? "").trim().toLowerCase());
</script>

<section aria-label="Project agent sessions">
  {#if unavailable.length}<p role="status">
      {unavailable.length} linked repositories require separate access. Their sessions
      are unavailable.
    </p>{/if}
  {#each scopeHomes.filter((home) => home.error || home.cursor) as home (home.repository.id)}
    {#if home.error}<p role="alert">
        Project scope in {home.repository.name}: {home.error}
      </p>{/if}
    {#if home.cursor}<p role="status">
        Sessions cover loaded projects in {home.repository.name}. More project
        links remain.
      </p>
      <Button
        variant="outline"
        disabled={hub.loading}
        onclick={() => hub.restore()}>Load more project links</Button
      >{/if}
  {/each}
  {#if !hub.loading && !hub.agents.homes.length}<Empty.Root
      ><Empty.Header
        ><Empty.Media variant="icon"><BotIcon aria-hidden="true" /></Empty.Media
        ><Empty.Title>No project repositories available</Empty.Title
        ><Empty.Description
          >Link an accessible repository to a project to see its captured agent
          sessions.</Empty.Description
        ></Empty.Header
      ></Empty.Root
    >{/if}
  {#each hub.agents.homes as home (home.repository.id)}
    <section
      aria-label={`Agents in ${home.repository.name}`}
      aria-busy={home.loading}
    >
      <StoreWorkCollection
        id={`agents:${home.repository.id}`}
        title={home.repository.name}
        count={home.sessions.length}
        Icon={GitBranchIcon}
      >
        {#if home.unsupported}<Empty.Root
            ><Empty.Header
              ><Empty.Media variant="icon"
                ><BotIcon aria-hidden="true" /></Empty.Media
              ><Empty.Title>Agents unavailable</Empty.Title><Empty.Description
                >This repository does not support recorded agent sessions.</Empty.Description
              ></Empty.Header
            ></Empty.Root
          >
        {:else if home.error}<p role="alert">{home.error}</p>
          <Button variant="outline" onclick={() => hub.refresh()}
            >Retry agent sessions</Button
          >{/if}
        {#if home.loading && !home.snapshot}<StoreCollaborationSkeleton
            label="Loading agent sessions"
            agents
          />
        {:else if home.snapshot}
          <div
            class="store-agent-session-list"
            role="list"
            aria-label={`Agent sessions in ${home.repository.name}`}
          >
            <CommunityProjectsVirtualRows
              rows={home.sessions.filter(
                (session) =>
                  !query ||
                  `${agentMember(session).name} ${session.state} ${session.session_id}`
                    .toLowerCase()
                    .includes(query),
              )}
              getKey={(session) => session.session_id}
              estimateSize={() => 82}
              hasMore={!!home.cursor}
              loading={home.loading}
              loadingAsListItem
              loadingLabel="Loading more agent sessions"
              onLoadMore={() => void hub.agents.more(home.repository.id)}
            >
              {#snippet row(session)}<StoreAgentSessionRow
                  {session}
                  collection
                  selectedId={route.workAgentRepository === home.repository.id
                    ? route.agentSessionId
                    : null}
                  onSelect={(id) => onSelect(home.repository.id, id)}
                />{/snippet}
            </CommunityProjectsVirtualRows>
          </div>
          {#if !home.sessions.length && !home.error}<Empty.Root
              ><Empty.Header
                ><Empty.Media variant="icon"
                  ><BotIcon aria-hidden="true" /></Empty.Media
                ><Empty.Title>No agent sessions</Empty.Title><Empty.Description
                  >Captured agent sessions for this repository will appear here.</Empty.Description
                ></Empty.Header
              ></Empty.Root
            >{/if}
          {#if home.cursor}<Button
              variant="outline"
              disabled={home.loading}
              onclick={() => hub.agents.more(home.repository.id)}
              >More agent sessions</Button
            >{/if}
        {/if}
      </StoreWorkCollection>
    </section>
  {/each}
</section>
