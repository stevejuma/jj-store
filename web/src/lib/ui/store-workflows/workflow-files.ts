import type { StoreDataSource } from "../../controller/native-data-source.js";
import {
  snapshotOperation,
  snapshotRepository,
  isLocal,
} from "../../controller/repository-profile.js";
/** File suggestions are operation-pinned reads; execution always resolves the recorded run revision. */
export async function workflowFiles(
  source: StoreDataSource,
  signal?: AbortSignal,
) {
  const head = await source.snapshot(true);
  const operation = head.heads[0];
  if (
    isLocal(head) ||
    !operation ||
    snapshotRepository(head) !== source.repository
  )
    throw Error("Native workflow source unavailable.");
  const log = await source.graph("log", {
    operation,
    limit: 1,
    atTimeMillis: Date.now(),
    signal,
  });
  if (
    log.data.data.graph !== "log" ||
    snapshotRepository(log.snapshot) !== source.repository ||
    snapshotOperation(log.snapshot) !== operation
  )
    throw Error("Workflow source revision changed.");
  const commit = log.data.data.nodes[0]?.commit.id;
  if (!commit) return [];
  const files: { id: string; name: string; description: string }[] = [];
  const cursors = new Set<string>();
  let cursor: string | undefined;
  do {
    const page = await source.paths(commit, operation, {
      path: ".github/workflows",
      recursive: true,
      limit: 256,
      cursor,
      signal,
    });
    if (
      snapshotRepository(page.snapshot) !== source.repository ||
      snapshotOperation(page.snapshot) !== operation
    )
      throw Error("Workflow file snapshot changed.");
    files.push(
      ...page.data.entries
        .filter((entry) => /^\.github\/workflows\/.+\.ya?ml$/.test(entry.path))
        .map((entry) => ({
          id: entry.path,
          description: entry.path,
          name: entry.path.slice(".github/workflows/".length),
        })),
    );
    if (files.length > 256)
      throw Error("Workflow file discovery exceeds 256 files.");
    cursor = page.next_cursor ?? undefined;
    if (cursor && cursors.has(cursor))
      throw Error("Workflow file continuation did not advance.");
    if (cursor) cursors.add(cursor);
  } while (cursor);
  return files;
}
