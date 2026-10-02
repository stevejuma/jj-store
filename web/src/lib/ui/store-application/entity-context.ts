import { provideDisclosureLayout } from "../disclosure-layout.svelte.js";
import type { StoreController } from "../../controller/contracts.js";
import { storeRouteToHash } from "../../controller/store-route-codec.js";
import { emptyRoute } from "../../controller/global-search-presentation.js";
import { communityEntityLinkPresentation } from "../../community/community-link-normalize.js";
import { provideCommunityEntityNavigation } from "../community-entity-chip/navigation.js";
import { provideCommunityComposerSlashCatalog } from "../community-chat-composer/composer-slash-context.js";
import {
  emptyComposerSlashSession,
  type ComposerSlashEntry,
} from "../community-chat-composer/composer-slash-session.js";
import type { ComposerSlashKind } from "../community-chat-composer/composer-slash.js";
export function provideStoreEntityContext(
  controller: () => StoreController,
  blocked: () => boolean,
  storage?: Storage,
) {
  provideDisclosureLayout(storage, () => controller().state.route);
  provideCommunityEntityNavigation({
    open: (link) => {
      if (!blocked()) void controller().actions.openRoute?.(link.route);
    },
    label: (link) => {
      const app = controller();
      const repository = app.state.repositories.find(
        (r) => r.id === link.route.repository,
      )?.name;
      if (link.kind === "channel")
        return (
          app.threads?.threads.find(
            (t) => t.value.thread_id === link.route.threadId,
          )?.value.title ?? "Thread"
        );
      return link.kind === "repo"
        ? `${repository ?? "Repository"} · ${communityEntityLinkPresentation(link).fallbackLabel}`
        : communityEntityLinkPresentation(link).fallbackLabel;
    },
  });
  provideCommunityComposerSlashCatalog({
    catalog: () => {
      const app = controller(),
        route = app.state.route;
      const entry = (
        id: string,
        label: string,
        route: Parameters<typeof storeRouteToHash>[0],
        detail?: string,
      ): ComposerSlashEntry => ({
        id,
        label,
        href: storeRouteToHash(route) ?? "",
        detail,
      });
      return {
        ...emptyComposerSlashSession(),
        repositories: app.state.repositories.map((r) =>
          entry(r.id, r.name, { ...emptyRoute(r.id), page: "log" }),
        ),
        threads:
          app.threads?.threads.map((t) =>
            entry(t.value.thread_id, t.value.title, {
              ...emptyRoute(route.repository),
              threadId: t.value.thread_id,
            }),
          ) ?? [],
        commits: app.state.targets.map((id) =>
          entry(
            id,
            id.slice(0, 12),
            { ...route, page: "log", commitId: id },
            "Commit",
          ),
        ),
        logs: app.state.operationHeads.map((id) =>
          entry(
            id,
            `Operation ${id.slice(0, 12)}`,
            { ...route, page: "oplog", inspectedOperationId: id },
            "Operation log",
          ),
        ),
      };
    },
    search: async (kind: ComposerSlashKind, query, signal) => {
      const search = controller().referenceSearch;
      if (!search || signal.aborted || kind === "repositories") return [];
      const cancel = () => search.clear();
      signal.addEventListener("abort", cancel, { once: true });
      try {
        const section = kind === "commits" || kind === "logs" ? "code" : kind;
        await search.run(query, { label: kind, section });
        if (signal.aborted) return [];
        const wanted = {
          threads: "thread",
          projects: "project",
          tasks: "task",
          commits: "commit",
          logs: "operation",
          repositories: "repository",
        }[kind];
        return search.results
          .filter((r) => r.kind === wanted)
          .slice(0, 24)
          .map((r) => ({
            id: r.id,
            label: r.title,
            detail: r.source,
            href: storeRouteToHash(r.route) ?? "",
          }));
      } finally {
        signal.removeEventListener("abort", cancel);
      }
    },
  });
}
