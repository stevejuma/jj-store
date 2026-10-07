export function displayValue(value: string | undefined): unknown {
  if (value === undefined) return undefined;
  let current: unknown = value;
  for (let index = 0; index < 3; index += 1) {
    const parsed = parseJson(current);
    if (parsed === undefined) break;
    current = parsed;
  }
  return current;
}
export function outputText(value: unknown): string | undefined {
  if (typeof value === "string") return value;
  const source = record(value);
  if (!source) return;
  for (const key of [
    "stdout",
    "formatted_output",
    "aggregated_output",
    "output",
    "text",
    "content",
    "result",
  ]) {
    if (typeof source[key] === "string") return source[key] as string;
  }
}
export function stderrText(value: unknown): string | undefined {
  return stringValue(record(value)?.stderr);
}
export function queryFromValue(value: unknown): string | undefined {
  const source = record(value);
  return stringValue(source?.query ?? source?.pattern);
}
export function queryFromCommand(command: string): string | undefined {
  const quoted = /['"]([^'"]+)['"]/u.exec(command);
  if (quoted?.[1]) return quoted[1];
  const parts = command
    .trim()
    .split(/\s+/u)
    .filter((part) => !part.startsWith("-"));
  return parts.length > 1 ? parts[1] : undefined;
}
export function patchText(value: unknown): string | undefined {
  if (typeof value !== "string") return;
  if (/^diff --git /mu.test(value) || /^@@ /mu.test(value)) return value;
}
export function pathFromRecord(value: Record<string, unknown> | undefined) {
  return stringValue(
    value?.path ??
      value?.file ??
      value?.filePath ??
      value?.file_path ??
      value?.target,
  );
}
export function pathFromPatch(patch: string): string | undefined {
  return (
    /diff --git a\/(.+?) b\//u.exec(patch)?.[1] ??
    /^\+\+\+ b\/(.+)$/mu.exec(patch)?.[1]
  );
}
export function arrayFromRecord(
  value: unknown,
  key: string,
): unknown[] | undefined {
  const items = record(value)?.[key];
  return Array.isArray(items) ? items : undefined;
}
export function parseJson(value: unknown): unknown {
  if (typeof value !== "string") return undefined;
  const trimmed = value.trim();
  if (!/^[{["]/u.test(trimmed)) return undefined;
  try {
    return JSON.parse(trimmed);
  } catch {
    return undefined;
  }
}
export function record(value: unknown): Record<string, unknown> | undefined {
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : undefined;
}
export function stringValue(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() ? value : undefined;
}
export function normalized(value: string): string {
  return value.trim().toLowerCase().split(/[:/]/u).at(-1) ?? "";
}
