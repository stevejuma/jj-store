import type { FreshnessChange } from "../api/application-events.js";
import type { StoreRoute } from "./contracts.js";
export type ProjectDomain =
  "work" | "agents" | "reviews" | "activity" | "directory";
const all: ProjectDomain[] = [
  "work",
  "agents",
  "reviews",
  "activity",
  "directory",
];
export function projectDomains(route: StoreRoute): ProjectDomain[] {
  const tab = route.workTab;
  const domain: ProjectDomain =
    tab === "repositories"
      ? "directory"
      : tab === "agents" || tab === "reviews" || tab === "activity"
        ? tab
        : "work";
  return route.projectId && domain !== "work" ? [domain, "work"] : [domain];
}
/** Each hidden projection retains its own dirtiness until a successful current read. */
export class ProjectsDirtiness {
  private sequence = 0;
  private pending = new Map<
    string,
    { id: string; domain: ProjectDomain; revision: number }
  >();
  record(changes: readonly FreshnessChange[], repositories: readonly string[]) {
    for (const change of changes) {
      if (
        change.repository === null &&
        !["repository", "discovery", "workspaces", "conversion"].includes(
          change.domain,
        )
      )
        continue;
      const ids =
        change.repository === null ? repositories : [change.repository];
      const domains =
        change.repository === null ||
        ["repository", "work"].includes(change.domain)
          ? all
          : change.domain === "code"
            ? (["activity", "directory"] as const)
            : change.domain === "agents"
              ? (["agents", "activity", "directory"] as const)
              : change.domain === "reviews"
                ? (["reviews", "directory"] as const)
                : change.domain === "threads"
                  ? (["work"] as const)
                  : [];
      for (const id of ids)
        for (const domain of domains)
          this.pending.set(`${id}:${domain}`, {
            id,
            domain,
            revision: ++this.sequence,
          });
    }
    const allowed = new Set(repositories);
    for (const [key, value] of this.pending)
      if (!allowed.has(value.id)) this.pending.delete(key);
  }
  capture(route: StoreRoute) {
    const domains = projectDomains(route);
    const captured = [...this.pending].filter(
      ([, value]) =>
        domains.includes(value.domain) &&
        (value.domain !== "work" ||
          !route.projectId ||
          value.id === route.repository),
    );
    return {
      affected: new Set(captured.map(([, value]) => value.id)),
      work: new Set(
        captured
          .filter(([, value]) => value.domain === "work")
          .map(([, value]) => value.id),
      ),
      accept: (successful: (domain: ProjectDomain, id: string) => boolean) => {
        for (const [key, value] of captured)
          if (
            this.pending.get(key)?.revision === value.revision &&
            successful(value.domain, value.id)
          )
            this.pending.delete(key);
      },
    };
  }
  clear() {
    this.pending.clear();
  }
}
