export type StorePage = "branches" | "log" | "oplog";
export interface CommunityNavigationItemModel {
  id: string;
  label: string;
  kind: StorePage | "repository";
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
  _section: CommunityNavigationSectionModel,
) {
  return "repositories";
}
