<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import type { StorePage } from "../community-navigation/model.js";
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
  const selectedBookmark = $derived(
    view.route.page === "branches"
      ? view.bookmarks.find((bookmark) => bookmark.name === view.route.bookmark)
      : undefined,
  );
  const selectedLabel = $derived(
    view.route.page === "oplog" ? "Selected operation" : "Selected change",
  );
  const contextTitle = $derived(repositoryContextTitle(view.route.page));
  const primaryAction = $derived(
    view.route.page === "branches"
      ? ({ action: "bookmark-create", label: "Add bookmark" } as const)
      : view.route.page === "log" || view.route.page === "files"
        ? ({ action: "new", label: "Add change" } as const)
        : undefined,
  );
  const bookmarkStatus = $derived(
    !selectedBookmark
      ? ""
      : selectedBookmark.targets.length === 0
        ? "Deleted"
        : selectedBookmark.targets.length > 1
          ? "Conflicted"
          : selectedBookmark.workspace
            ? "Workspace"
            : selectedBookmark.default
              ? "Default"
              : "Local",
  );

  function repositoryContextTitle(page: StorePage): string {
    if (page === "branches") return "Branches/Bookmarks";
    if (page === "log") return "JJ Log/Changes";
    if (page === "oplog") return "Op Log/Operations";
    if (page === "files") return "Files/Repository";
    if (page === "agents") return "Agents/Sessions";
    return "Reviews/Threads";
  }
</script>

<div class="store-repository-details">
  <div class="store-repository-details__context-heading">
    <strong>{contextTitle}</strong>
    {#if primaryAction}<StoreRepositoryActions
        {controller}
        presentation="primary"
        primaryAction={primaryAction.action}
        primaryLabel={primaryAction.label}
      />{/if}
  </div>
  {#if selected}
    <section aria-label={selectedLabel}>
      <p class="store-repository-details__label">{selectedLabel}</p>
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
  {:else if selectedBookmark}
    <section aria-label="Selected bookmark">
      <p class="store-repository-details__label">Selected bookmark</p>
      <p class="store-repository-details__message">{selectedBookmark.name}</p>
      <p class="store-repository-details__description">
        {selectedBookmark.description ?? "No description"}
      </p>
      {#if selectedBookmark.author}<div
          class="store-repository-details__author"
        >
          <CommunityProjectsGitAuthorAvatar
            email={selectedBookmark.authorEmail}
            name={selectedBookmark.author}
            size="small"
          />
          <span>{selectedBookmark.author}</span>
          {#if selectedBookmark.updatedAt}<CommunityRelativeTime
              timestamp={selectedBookmark.updatedAt}
              now={view.now}
            />{/if}
        </div>{/if}
      <dl>
        <div>
          <dt>Status</dt>
          <dd>{bookmarkStatus}</dd>
        </div>
        <div>
          <dt>Targets</dt>
          <dd>
            {selectedBookmark.targets.length}
            {selectedBookmark.targets.length === 1 ? "target" : "targets"}
          </dd>
        </div>
      </dl>
      {#if selectedBookmark.targets.length === 1}<CommunityProjectsRevisionGraphIds
          commitId={selectedBookmark.targets[0]}
          placement="inline"
        />{/if}
    </section>
  {:else}
    <section aria-label="Repository context">
      <dl>
        <div>
          <dt>Repository</dt>
          <dd>{view.repository.name}</dd>
        </div>
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
  {#if selected || selectedBookmark}
    <section aria-label="Actions">
      <p class="store-repository-details__label">Actions</p>
      <StoreRepositoryActions {controller} presentation="panel" />
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

  .store-repository-details__context-heading {
    display: grid;
    min-height: 2rem;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 0.5rem;
    padding: 0 0.4rem 0.55rem;
    font-size: var(--community-text-secondary);
    font-weight: 500;
  }

  .store-repository-details__context-heading strong {
    overflow: hidden;
    font-weight: 500;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .store-repository-details__context-heading
    :global(> [data-ui-component="button"]) {
    width: 1.75rem;
    min-width: 1.75rem;
    height: 1.75rem;
    min-height: 1.75rem;
    margin-inline-end: -0.4rem;
  }

  .store-repository-details__context-heading
    :global(> [data-ui-component="button"]:is(:hover, :focus-visible)) {
    color: var(--foreground);
    background: var(--muted);
  }

  .store-repository-details__label {
    margin: 0.9rem 0.4rem 0.3rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .store-repository-details__message,
  .store-repository-details__description {
    margin: 0;
    padding-inline: 0.4rem;
    overflow-wrap: anywhere;
    font-size: var(--community-text-body);
  }

  .store-repository-details__message {
    font-weight: 550;
  }

  .store-repository-details__description {
    color: var(--muted-foreground);
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
