import { ApiError, pretty } from "../api/client.js";
import type {
  ProjectHome,
  WorkBrowseQuery,
  WorkBrowseTransport,
} from "./projects-hub-types.js";

export async function readProjectHome(
  context: {
    transport(home: string): WorkBrowseTransport;
    signal: AbortSignal;
    current(): boolean;
    denied(home: string): void;
  },
  home: ProjectHome,
  query: WorkBrowseQuery,
  more = false,
) {
  home.loading = true;
  home.error = "";
  try {
    const page = await context.transport(home.repository.id).browse(
      {
        ...query,
        limit: 50,
        ...(more ? { cursor: home.cursor ?? undefined } : {}),
      },
      context.signal,
    );
    if (!context.current()) return;
    if (more && pretty(page.snapshot) !== pretty(home.snapshot))
      throw new ApiError("snapshot_mismatch");
    home.projects = more ? [...home.projects, ...page.projects] : page.projects;
    home.tasks = more ? [...home.tasks, ...page.tasks] : page.tasks;
    home.snapshot = page.snapshot;
    home.cursor = page.next_cursor;
    home.total = Number(page.total_matches);
  } catch (cause) {
    if (
      context.current() &&
      cause instanceof ApiError &&
      ["access_denied", "bearer_required"].includes(cause.code)
    ) {
      context.denied(home.repository.id);
      home.projects = [];
      home.tasks = [];
      home.snapshot = null;
      home.cursor = null;
    }
    if (context.current())
      home.error = cause instanceof Error ? cause.message : "Home unavailable";
  } finally {
    if (context.current()) home.loading = false;
  }
}

export async function reloadProjectHomeWindow(
  home: ProjectHome,
  read: (more?: boolean) => Promise<void>,
  refresh: boolean,
  current: () => boolean,
) {
  const count = Math.min(home.projects.length + home.tasks.length, 4096);
  await read();
  const seen = new Set<string>();
  while (
    refresh &&
    home.cursor &&
    !home.error &&
    current() &&
    home.projects.length + home.tasks.length < count
  ) {
    if (seen.has(home.cursor)) throw new Error("Repeated WorkStore cursor.");
    seen.add(home.cursor);
    await read(true);
  }
}
