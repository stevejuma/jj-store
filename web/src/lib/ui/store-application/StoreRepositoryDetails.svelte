<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import StoreRepositoryActions from "../store-mutations/StoreRepositoryActions.svelte";

  let { controller, title }: { controller: StoreController; title: string } =
    $props();
  const view = $derived(controller.state);
  const selectedId = $derived(
    view.route.page === "oplog"
      ? view.route.inspectedOperationId
      : view.route.commitId,
  );
  const selected = $derived(
    view.graph.revisions.find((revision) => revision.eventId === selectedId),
  );
  const selectedLabel = $derived(
    view.route.page === "oplog" ? "Selected operation" : "Selected change",
  );
</script>

<div class="store-repository-details">
  {#if selected}
    <section aria-label={selectedLabel}>
      <div class="store-repository-details__heading">
        <strong>{selectedLabel}</strong>
      </div>
      <p class="store-repository-details__message">{selected.message}</p>
      <div class="store-repository-details__author">
        <CommunityProjectsGitAuthorAvatar
          email={selected.authorEmail}
          name={selected.author}
          size="small"
        />
        <span>{selected.author}</span>
        <CommunityRelativeTime timestamp={selected.createdAt} now={view.now} />
      </div>
      <CommunityProjectsRevisionGraphIds
        changeId={view.route.page === "oplog" ? undefined : selected.changeId}
        commitId={selected.eventId}
        commitLabel={view.route.page === "oplog" ? "operation ID" : "commit ID"}
        placement="inline"
      />
    </section>
    <section aria-label="Actions">
      <p class="store-repository-details__label">Actions</p>
      <StoreRepositoryActions {controller} presentation="panel" />
    </section>
  {:else}
    <section aria-label="Repository context">
      <div class="store-repository-details__heading">
        <strong>{view.repository.name}</strong>
      </div>
      <dl>
        <div>
          <dt>View</dt>
          <dd>{title}</dd>
        </div>
        <div>
          <dt>Source</dt>
          <dd>
            {view.status.connection === "connected"
              ? "Authenticated native repository"
              : "In-memory presentation"}
          </dd>
        </div>
      </dl>
    </section>
  {/if}
</div>

<style>
  .store-repository-details {
    display: grid;
    width: 100%;
    gap: 0.15rem;
    padding: 0.85rem 0.75rem;
    box-sizing: border-box;
  }

  .store-repository-details section {
    display: grid;
    min-width: 0;
    gap: 0.55rem;
  }

  .store-repository-details__heading {
    min-height: 2rem;
    padding: 0 0.4rem 0.55rem;
    font-size: var(--community-text-secondary);
    font-weight: 500;
  }

  .store-repository-details__heading strong {
    font-weight: 500;
  }

  .store-repository-details__label {
    margin: 0.9rem 0.4rem 0.3rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .store-repository-details__message {
    margin: 0;
    padding-inline: 0.4rem;
    overflow-wrap: anywhere;
    font-size: var(--community-text-body);
    font-weight: 550;
  }

  .store-repository-details__author {
    display: flex;
    min-width: 0;
    align-items: center;
    gap: 0.4rem;
    padding-inline: 0.4rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .store-repository-details :global(.community-revision-graph-ids) {
    padding-inline: 0.4rem;
  }

  dl {
    display: grid;
    gap: 0.65rem;
    margin: 0;
    padding: 0 0.4rem;
    font-size: var(--community-text-body);
  }

  dl div {
    display: grid;
    gap: 0.15rem;
  }

  dt {
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }
</style>
