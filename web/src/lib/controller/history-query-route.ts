import type { StorePage } from "../ui/community-navigation/model.js";

/** Only successfully applied history text and its frozen date context enter URLs. */
export function historyQueryFromParams(
  query: URLSearchParams,
  page: StorePage,
) {
  const texts = query.getAll("history_query"),
    times = query.getAll("history_time");
  if (texts.length > 1 || times.length > 1) return null;
  const historyQuery = texts[0] ?? null,
    historyTime = times[0] ?? null;
  if (
    (historyQuery !== null || historyTime !== null) &&
    !["log", "oplog"].includes(page)
  )
    return null;
  if (
    historyQuery !== null &&
    (!historyQuery.trim() ||
      /[\u0000-\u001f]/u.test(historyQuery) ||
      new TextEncoder().encode(historyQuery).length > 4096)
  )
    return null;
  if (
    historyTime !== null &&
    (!/^\d{1,16}$/.test(historyTime) ||
      !Number.isSafeInteger(Number(historyTime)) ||
      !Number.isFinite(new Date(Number(historyTime)).valueOf()))
  )
    return null;
  return { historyQuery, historyTime };
}

export function historyQueryToParams(
  query: URLSearchParams,
  route: import("./contracts.js").StoreRoute,
) {
  if (!["log", "oplog"].includes(route.page)) return;
  if (route.historyQuery) query.set("history_query", route.historyQuery);
  if (route.historyTime !== undefined)
    query.set("history_time", String(route.historyTime));
}
