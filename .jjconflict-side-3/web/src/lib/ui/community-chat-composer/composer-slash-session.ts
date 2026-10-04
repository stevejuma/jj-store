import type { CommunityRoomModel } from "../community-workspace/model.js";

export interface ComposerSlashEntry {
  id: string;
  label: string;
  href: string;
  detail?: string;
}

export interface ComposerSlashSession {
  rooms: readonly CommunityRoomModel[];
  threads: readonly ComposerSlashEntry[];
  projects: readonly ComposerSlashEntry[];
  repositories: readonly ComposerSlashEntry[];
  tasks: readonly ComposerSlashEntry[];
  commits: readonly ComposerSlashEntry[];
  logs: readonly ComposerSlashEntry[];
}

export function emptyComposerSlashSession(): ComposerSlashSession {
  return {
    rooms: [],
    threads: [],
    projects: [],
    repositories: [],
    tasks: [],
    commits: [],
    logs: [],
  };
}
