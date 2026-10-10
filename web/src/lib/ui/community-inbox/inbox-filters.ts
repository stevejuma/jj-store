import type { SessionInboxItem } from "./model.js";
export const inboxFacets = [
  "repositories",
  "agents",
  "statuses",
  "harnesses",
] as const;
export type InboxFacet = (typeof inboxFacets)[number];
export type InboxFilters = Record<InboxFacet, readonly string[]>;
export const emptyInboxFilters = (): InboxFilters => ({
  repositories: [],
  agents: [],
  statuses: [],
  harnesses: [],
});
export const facetLabels: Record<InboxFacet, string> = {
  repositories: "Repository",
  agents: "Agent",
  statuses: "Status",
  harnesses: "Harness",
};
export function inboxFilterOptions(
  items: readonly SessionInboxItem[],
  facet: InboxFacet,
) {
  const options = new Map<string, string>();
  for (const item of items) {
    if (facet === "repositories")
      options.set(
        item.repository?.id ?? "",
        item.repository?.name ?? "No repository",
      );
    if (facet === "agents")
      for (const agent of item.agents ?? []) options.set(agent.id, agent.name);
    if (facet === "statuses") options.set(item.status, item.status);
    if (facet === "harnesses" && item.harness)
      options.set(item.harness, item.harness);
  }
  return [...options]
    .map(([id, label]) => ({ id, label }))
    .sort((a, b) => a.label.localeCompare(b.label) || a.id.localeCompare(b.id));
}
/** OR within a facet, AND across facets; only filter loaded authorized rows. */
export function filterInboxItems(
  items: readonly SessionInboxItem[],
  query: string,
  filters: InboxFilters,
  view = "all",
) {
  const search = query.trim().toLocaleLowerCase();
  return items.filter(
    (item) =>
      (view !== "active" ||
        !["archived", "tombstoned"].includes(item.status)) &&
      (view !== "archived" || item.status === "archived") &&
      (view !== "connected" ||
        ["Working", "Online", "Starting"].includes(item.status)) &&
      [
        item.label,
        item.author.name,
        item.context,
        item.excerpt,
        item.repository?.name,
        ...(item.agents ?? []).map((a) => a.name),
      ].some((text) => text?.toLocaleLowerCase().includes(search)) &&
      (!filters.repositories.length ||
        filters.repositories.includes(item.repository?.id ?? "")) &&
      (!filters.agents.length ||
        item.agents?.some((a) => filters.agents.includes(a.id))) &&
      (!filters.statuses.length || filters.statuses.includes(item.status)) &&
      (!filters.harnesses.length ||
        (item.harness && filters.harnesses.includes(item.harness))),
  );
}
