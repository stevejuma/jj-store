export type StorePage =
  | "branches"
  | "log"
  | "oplog"
  | "files"
  | "agents"
  | "reviews"
  | "projects"
  | "threads";
export interface CommunityNavigationItemModel {
  id: string;
  label: string;
  kind:
    | StorePage
    | "channel"
    | "repository"
    | "project"
    | "more-projects"
    | "retry-projects";
  home?: string;
  project?: string;
  thread?: string;
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
  return section.id === "threads"
    ? "channels"
    : section.id === "projects"
      ? "projects"
      : "repositories";
}
