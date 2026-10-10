import { readQuery } from "./read-query.js";
import { createStoreQueryClient } from "./store-query-client.js";
import {
  readAgentWorkspaceChanges,
  type AgentChangeSelection,
} from "./agent-workspace-changes.js";
/** Installation-owned immutable previews; observers share transport and release independently. */
export class AgentChangesCache {
  private queries = createStoreQueryClient();
  private retained = new Map<string, number>();
  private closed = false;
  constructor(
    private fetcher: typeof fetch,
    private capture = readAgentWorkspaceChanges,
  ) {}
  async read(selection: AgentChangeSelection, signal: AbortSignal) {
    if (this.closed) throw new DOMException("Closed", "AbortError");
    const key = JSON.stringify([
      selection.repository,
      selection.profile,
      selection.operation,
      selection.base_commit,
      selection.commit,
    ]);
    const value = await readQuery(
      this.queries,
      {
        queryKey: ["agent-changes", key],
        staleTime: Infinity,
        retry: false,
        queryFn: ({ signal }) => this.capture(selection, this.fetcher, signal),
      },
      signal,
    );
    if (this.closed) throw new DOMException("Closed", "AbortError");
    this.retained.delete(key);
    this.retained.set(key, JSON.stringify(value).length);
    while (
      this.retained.size > 8 ||
      [...this.retained.values()].reduce((sum, size) => sum + size, 0) >
        8 * 1024 * 1024
    ) {
      const oldest = this.retained.keys().next().value!;
      this.retained.delete(oldest);
      this.queries.removeQueries({
        queryKey: ["agent-changes", oldest],
        exact: true,
      });
    }
    return value;
  }
  destroy() {
    this.closed = true;
    void this.queries.cancelQueries();
    this.queries.clear();
    this.retained.clear();
  }
}
