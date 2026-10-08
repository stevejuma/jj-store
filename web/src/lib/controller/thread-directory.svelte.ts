import { ApiError, type Model } from "../api/client.js";
import type { ThreadTransport } from "./thread-types.js";
export function createThreadDirectory(options: {
  source: () => ThreadTransport;
  generation: () => number;
  valid: (ticket: number, transport: ThreadTransport) => boolean;
  status: (value: string) => void;
}) {
  let threads = $state<Model<"ThreadSummaryV1">[]>([]),
    listCursor = $state<string | null>(null),
    directoryLoading = $state(false),
    directoryError = $state("");
  async function list(more = false) {
    const transport = options.source(),
      ticket = options.generation();
    directoryLoading = true;
    options.status(threads.length ? "refreshing" : "loading");
    try {
      const p = await transport.list(
        more ? (listCursor ?? undefined) : undefined,
      );
      if (!options.valid(ticket, transport)) return;
      if (p.items.type !== "threads") throw new ApiError("invalid_thread_page");
      threads = more ? [...threads, ...p.items.threads] : p.items.threads;
      listCursor = p.next_cursor;
      options.status("ready");
      directoryError = "";
    } catch (e) {
      if (options.valid(ticket, transport)) {
        options.status("failed");
        directoryError = String(e instanceof Error ? e.message : e);
      }
    } finally {
      if (options.valid(ticket, transport)) directoryLoading = false;
    }
  }
  return {
    list,
    get threads() {
      return threads;
    },
    get hasMore() {
      return !!listCursor;
    },
    get loading() {
      return directoryLoading;
    },
    set loading(value: boolean) {
      directoryLoading = value;
    },
    get error() {
      return directoryError;
    },
    clear() {
      threads = [];
      listCursor = null;
      directoryLoading = false;
      directoryError = "";
    },
  };
}
