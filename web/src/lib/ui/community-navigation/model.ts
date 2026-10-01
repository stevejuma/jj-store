export type StorePage =
  "branches" | "log" | "oplog" | "files" | "agents" | "reviews" | "projects";
export interface CommunityNavigationItemModel {
  id: string;
  label: string;
  kind:
    StorePage | "repository" | "project" | "more-projects" | "retry-projects";
  home?: string;
  project?: string;
  children?: readonly CommunityNavigationItemModel[];
  initiallyExpanded?: boolean;
}
export interface CommunityNavigationSectionModel {
  id: string;
  label: string;
  items: readonly CommunityNavigationItemModel[];
  initiallyCollapsed?: boolean;
}
export interface CommunityNavigationModel {
  primaryItems: readonly CommunityNavigationItemModel[];
  sections: readonly CommunityNavigationSectionModel[];
}
export function communityNavigationSectionKind(
  section: CommunityNavigationSectionModel,
) {
  return section.id === "projects" ? "projects" : "repositories";
}
