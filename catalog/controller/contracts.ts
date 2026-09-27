import type { CommunityMemberModel } from "../components/community-workspace/model.js";
import type { StorePage } from "../components/community-navigation/model.js";
import type { RepositoryLogGraph } from "../projects/repository-log.js";
import type { Bookmark } from "../projects/presentation.js";
export interface RepositoryBlob {
  path: string;
  contents?: string;
  binary?: boolean;
  too_large?: boolean;
}
export interface RepositorySnapshot {
  id: string;
  files: readonly RepositoryBlob[];
}
export interface RepositoryChoice {
  value: string;
  label: string;
  kind: "bookmark" | "change";
  searchTerms?: readonly string[];
  targets: readonly string[];
}
export interface StoreRoute {
  page: StorePage;
  repository: string;
  bookmark: string;
  revisionId: string | null;
  path: string | null;
  directory: string;
}
export interface StoreControllerState {
  readonly route: StoreRoute;
  readonly canNavigateBack: boolean;
  readonly canNavigateForward: boolean;
  readonly repositories: readonly { id: string; name: string }[];
  readonly repository: { id: string; name: string };
  readonly graph: RepositoryLogGraph;
  readonly bookmarks: readonly Bookmark[];
  readonly choices: readonly RepositoryChoice[];
  readonly targets: readonly string[];
  readonly snapshot: RepositorySnapshot | undefined;
  readonly blob: RepositoryBlob | undefined;
  readonly authenticated: boolean;
  readonly member: CommunityMemberModel;
}
/** Community's state/actions contract shape, scoped to local JJ presentation. */
export interface StoreController {
  readonly state: StoreControllerState;
  readonly actions: {
    openRepository(id: string): void;
    selectPage(page: StorePage): void;
    navigate(delta: -1 | 1): void;
    selectBookmark(name: string): void;
    selectRevision(id: string | null): void;
    selectFile(path: string | null): void;
    selectDirectory(path: string): void;
    selectRef(value: string): void;
    signIn(): void;
    signOut(): void;
  };
}
