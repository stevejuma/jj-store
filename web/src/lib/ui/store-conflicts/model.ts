import type { Model } from "../../api/client.js";
import type { StoreMutationCommand } from "../../controller/store-mutation-types.js";

export type StoreConflictCommand = Extract<
  StoreMutationCommand,
  { type: "resolve_conflicts" }
>;
export type StoreConflictResolution =
  StoreConflictCommand["paths"][number]["resolution"];

export function buildConflictCommand(
  commit: string,
  conflicts: readonly import("../../controller/repository-read-types.js").CodeConflict[],
  selections: ReadonlyMap<string, StoreConflictResolution>,
): StoreConflictCommand {
  requireHex(commit, 128, "commit");
  if (conflicts.length === 0)
    throw new Error("No native conflicts to resolve.");
  const seen = new Set<string>();
  const paths = conflicts.map((conflict) => {
    if (seen.has(conflict.path))
      throw new Error(`Duplicate conflict path ${conflict.path}.`);
    seen.add(conflict.path);
    requireHex(conflict.fingerprint, 128, `${conflict.path} fingerprint`);
    const resolution = selections.get(conflict.path);
    if (!resolution)
      throw new Error(`Choose a resolution for ${conflict.path}.`);
    validateResolution(conflict, resolution);
    return {
      path: conflict.path,
      fingerprint: conflict.fingerprint,
      resolution: structuredClone(resolution),
    };
  });
  for (const path of selections.keys())
    if (!seen.has(path)) throw new Error(`Unknown conflict path ${path}.`);
  return { type: "resolve_conflicts", commit, paths };
}

function validateResolution(
  conflict: import("../../controller/repository-read-types.js").CodeConflict,
  resolution: StoreConflictResolution,
) {
  if (
    resolution.type === "select_term" &&
    !conflict.terms.some((term) => term.id === resolution.term)
  )
    throw new Error(
      `Conflict ${conflict.path} does not contain term ${resolution.term}.`,
    );
  if (resolution.type === "uploaded_file") {
    requireHex(resolution.upload, 64, "conflict upload");
    if (
      resolution.copy_from_term !== undefined &&
      resolution.copy_from_term !== null &&
      !conflict.terms.some((term) => term.id === resolution.copy_from_term)
    )
      throw new Error(
        `Conflict ${conflict.path} does not contain copy term ${resolution.copy_from_term}.`,
      );
  }
}

function requireHex(value: string, width: number, label: string) {
  if (!new RegExp(`^[0-9a-f]{${width}}$`, "u").test(value))
    throw new Error(`${label} must be a full native identifier.`);
}
