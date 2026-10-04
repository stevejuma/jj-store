import { requireCapture } from "./capture-response-validation.js";

export const coverageCounts = [
  "roots",
  "unchecked_roots",
  "available_roots",
  "unavailable_roots",
  "tracked",
  "observed_sources",
  "awaiting_activity",
  "caught_up",
  "running_sources",
  "degraded_sources",
];

export function validateConfiguredRoots(
  roots: unknown,
  count: number | bigint,
) {
  if (roots === undefined || roots === null) return;
  requireCapture(
    Array.isArray(roots) &&
      roots.length <= 16 &&
      BigInt(roots.length) === BigInt(count),
  );
  const paths = new Set<string>();
  for (const path of roots) {
    requireCapture(
      typeof path === "string" &&
        path.length >= 1 &&
        path.length <= 4096 &&
        !/[\u0000-\u001f\u007f-\u009f]/.test(path) &&
        /^(?:\/|[A-Za-z]:[\\/]|\\\\)/.test(path) &&
        !paths.has(path),
    );
    paths.add(path);
  }
}
