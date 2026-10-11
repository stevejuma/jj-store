export type StorePage =
  | "settings"
  | "stacks"
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
    | "add-repository"
    | "add-local-repository"
    | "create-server-repository"
    | "add-project"
    | "project"
    | "more-projects"
    | "more-threads"
    | "retry-threads"
    | "retry-projects";
  home?: string;
  project?: string;
  thread?: string;
  repository?: string;
  description?: string;
  disabled?: boolean;
  children?: readonly CommunityNavigationItemModel[];
  initiallyExpanded?: boolean;
}
export interface CommunityNavigationSectionModel {
  id: string;
  label: string;
  items: readonly CommunityNavigationItemModel[];
  initiallyCollapsed?: boolean;
  action?: CommunityNavigationItemModel;
  actions?: readonly CommunityNavigationItemModel[];
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
