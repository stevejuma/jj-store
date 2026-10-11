/** Presentation drafts supplement controller-owned pending-command guards. */
export const repositorySettingsGuardKey = Symbol("repository-settings-drafts");
export interface RepositorySettingsDraftGuard {
  folders: string;
}
