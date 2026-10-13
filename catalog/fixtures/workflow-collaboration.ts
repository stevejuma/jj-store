import { createThreadTransport } from "./thread-transport.js";
import type { StoreDataSource } from "../../web/src/lib/controller/native-data-source.js";

/** The workflow picker fixture supplies only its read surface. */
export function workflowCollaboration(repository: string): StoreDataSource {
  const { transport } = createThreadTransport();
  const reader: Pick<StoreDataSource, "repository" | "threads" | "close"> = {
    repository,
    close() {},
    threads: {
      ...transport,
      async list(cursor) {
        const page = await transport.list(cursor);
        return { ...page, snapshot: { ...page.snapshot, repo_id: repository } };
      },
    },
  };
  return reader as StoreDataSource;
}
