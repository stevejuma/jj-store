import { initialStoreRoute } from "./store-route-history.svelte.js";
import type { StoreRoute } from "./contracts.js";
const id = /^[0-9a-f]{32}$/u;
export function threadsRoute(
  home = "",
  thread: string | null = null,
  repository?: string,
): StoreRoute {
  return {
    ...initialStoreRoute(),
    page: "threads",
    threadsHub: true,
    repository: home,
    threadId: thread,
    ...(repository ? { threadRepository: repository } : {}),
  };
}
export function threadsRouteToHash(route: StoreRoute): string | null {
  if (
    route.repository &&
    (!id.test(route.repository) || !route.threadId || !id.test(route.threadId))
  )
    return null;
  if (route.threadRepository && !id.test(route.threadRepository)) return null;
  const query = new URLSearchParams();
  if (route.threadRepository) query.set("repository", route.threadRepository);
  if (route.replyId) {
    if (!id.test(route.replyId)) return null;
    query.set("reply", route.replyId);
  }
  return `#/threads${route.threadId ? `/${route.repository}/${route.threadId}` : ""}${query.size ? `?${query}` : ""}`;
}
export function readThreadsHash(hash: string) {
  if (!(
    hash === "#/threads" ||
    hash.startsWith("#/threads/") ||
    hash.startsWith("#/threads?")
  ))
    return;
  const [path, search] = hash.slice(1).split("?");
  const query = new URLSearchParams(search);
  const selected = /^\/threads\/([0-9a-f]{32})\/([0-9a-f]{32})$/u.exec(
    path ?? "",
  );
  const repository = query.get("repository"),
    reply = query.get("reply");
  if (
    (path !== "/threads" && !selected) ||
    [...query.keys()].some((key) => !["repository", "reply"].includes(key)) ||
    (repository && !id.test(repository)) ||
    (reply && (!selected || !id.test(reply)))
  )
    return { kind: "invalid" as const, message: "Invalid Threads route." };
  return {
    kind: "route" as const,
    route: {
      ...threadsRoute(selected?.[1], selected?.[2], repository ?? undefined),
      replyId: reply,
    },
  };
}
