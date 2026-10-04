export const workspaceNavigationKey = Symbol("jj-store workspace navigation");
export interface WorkspaceNavigationGuard {
  editing: boolean;
}
