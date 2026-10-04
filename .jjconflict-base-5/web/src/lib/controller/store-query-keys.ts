export interface StoreQueryScope {
  session: string;
  epoch?: string;
  repository: string;
  profile?: import("./repository-profile.js").RepositoryProfile;
}

type QueryValue = string | number | bigint | boolean | null | undefined;

export const storeQueryKeys = {
  root: (scope: StoreQueryScope) =>
    [
      "jj-store",
      scope.session,
      scope.profile ?? "native",
      scope.repository,
    ] as const,
  resource: (
    scope: StoreQueryScope,
    kind: string,
    snapshot: string | null,
    query: Readonly<Record<string, QueryValue>> = {},
  ) =>
    [
      ...storeQueryKeys.root(scope),
      kind,
      snapshot,
      query,
      snapshot ? (scope.epoch ?? null) : null,
    ] as const,
};
