import type { RepositorySnapshot } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";

const order = new Map([
  ["readme", 0],
  ["code_of_conduct", 1],
  ["contributing", 2],
  ["license", 3],
  ["licence", 3],
  ["security", 4],
]);

export function landingDocuments(files: ReadonlyMap<string, string>) {
  const selectedRanks = new Set<number>();
  return [...files]
    .map(([path, id]) => {
      const match =
        /^(readme|code_of_conduct|contributing|licen[cs]e|security)(?:\.[^/]*)?$/iu.exec(
          path,
        );
      return match
        ? { path, id, rank: order.get(match[1]!.toLowerCase())! }
        : null;
    })
    .filter((document): document is NonNullable<typeof document> =>
      Boolean(document),
    )
    .sort(
      (left, right) =>
        left.rank - right.rank || left.path.localeCompare(right.path),
    )
    .filter(({ rank }) => {
      if (selectedRanks.has(rank)) return false;
      selectedRanks.add(rank);
      return true;
    })
    .slice(0, 5);
}

export async function readLandingDocuments(
  source: StoreDataSource,
  operation: string,
  snapshot: RepositorySnapshot,
  files: ReadonlyMap<string, string>,
  signal?: AbortSignal,
): Promise<RepositorySnapshot> {
  const documents = landingDocuments(files);
  const loaded: Array<
    readonly [string, Awaited<ReturnType<StoreDataSource["file"]>>]
  > = [];
  for (const { path, id } of documents)
    if (
      !snapshot.files.some(
        (file) =>
          file.path === path && (file.contents !== undefined || file.binary),
      )
    ) {
      signal?.throwIfAborted();
      loaded.push([
        path,
        await source.file(id, operation, { commit: snapshot.id, path, signal }),
      ]);
    }
  const contents = new Map(loaded);
  return {
    ...snapshot,
    files: snapshot.files.map((file) => {
      const value = contents.get(file.path);
      return value
        ? { ...file, contents: value.text, binary: value.text === undefined }
        : file;
    }),
  };
}
