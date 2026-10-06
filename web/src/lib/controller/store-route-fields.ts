import type { StorePage } from "../ui/community-navigation/model.js";

export function safeLabel(value: string, maximum: number): boolean {
  return (
    value.length > 0 &&
    value.length <= maximum &&
    !/[\u0000-\u001f]/u.test(value)
  );
}

export function safePath(value: string): boolean {
  if (!safeLabel(value, 4096) || value.startsWith("/") || value.includes("\\"))
    return false;
  return value
    .split("/")
    .every((part) => part && part !== "." && part !== "..");
}

export function one(
  query: URLSearchParams,
  key: string,
): string | null | undefined {
  const values = query.getAll(key);
  return values.length > 1 ? undefined : (values[0] ?? null);
}

export function validOptional(
  value: string | null | undefined,
  pattern: RegExp,
): value is string | null {
  return value !== undefined && (value === null || pattern.test(value));
}

export const repositoryPages = new Set<StorePage>([
  "stacks",
  "branches",
  "log",
  "oplog",
  "files",
  "agents",
  "reviews",
  "projects",
  "threads",
]);
export const repositoryQueryKeys = new Set([
  "context",
  "stack_tab",
  "stack_history",
  "at_op",
  "history_query",
  "history_time",
  "bookmark",
  "workspace",
  "commit",
  "evolution",
  "change",
  "operation",
  "parent",
  "group",
  "file",
  "file_view",
  "directory",
  "session",
  "thread",
  "review_view",
  "project",
  "task",
  "channel",
  "reply",
  "comment",
]);
