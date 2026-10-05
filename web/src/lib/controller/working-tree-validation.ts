import { ApiError } from "../api/client.js";
import { safePath } from "./store-route-fields.js";
import type {
  WorkingTreeObservation,
  WorkingTreeSelection,
} from "./working-tree-types.js";

const encoder = new TextEncoder();
const id = (value: unknown, width: number) =>
  typeof value === "string" && new RegExp(`^[0-9a-f]{${width}}$`).test(value);
const text = (value: unknown, maximum: number, empty = false) =>
  typeof value === "string" &&
  (empty || value.length > 0) &&
  encoder.encode(value).length <= maximum &&
  new TextDecoder().decode(encoder.encode(value)) === value &&
  !/[\u0000-\u001f\u007f-\u009f]/u.test(value);
export function workingTreeInteger(
  value: unknown,
  maximum = 18446744073709551615n,
): bigint | null {
  if (
    (typeof value !== "bigint" && typeof value !== "number") ||
    (typeof value === "number" && !Number.isSafeInteger(value))
  )
    return null;
  const number = BigInt(value);
  return number >= 0n && number <= maximum ? number : null;
}
export function validateWorkingTreeSelection(value: WorkingTreeSelection) {
  if (!id(value.operation, 128) || !workingTreeInteger(value.bindingGeneration))
    throw new ApiError("invalid_working_tree_selection");
}
function fail(): never {
  throw new ApiError("invalid_working_tree_observation");
}
function record(value: unknown, keys: string[]): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) fail();
  const result = value as Record<string, unknown>;
  if (
    Object.keys(result).length !== keys.length ||
    keys.some((key) => !Object.hasOwn(result, key))
  )
    fail();
  return result;
}
function file(value: unknown) {
  const result = record(value, ["kind", "bytes", "content_digest"]);
  const kind = result.kind as Record<string, unknown> | null;
  if (kind?.type === "file") {
    record(kind, ["type", "executable"]);
    if (typeof kind.executable !== "boolean") fail();
  } else if (kind?.type === "symlink") record(kind, ["type"]);
  else fail();
  const bytes = workingTreeInteger(result.bytes, 2n * 1024n * 1024n);
  if (bytes === null || !id(result.content_digest, 64)) fail();
  return {
    bytes,
    digest: result.content_digest,
    type: kind!.type,
    executable: kind!.executable,
  };
}

/** Complete fail-closed bounds and identities, without narrowing native integers. */
export function validateWorkingTreeObservation(
  value: unknown,
  repository: string,
  selection: WorkingTreeSelection,
): WorkingTreeObservation {
  const result = record(value, ["snapshot", "changes", "obstacle_count"]);
  const pin = record(result.snapshot, [
    "version",
    "repository",
    "binding_generation",
    "operation",
    "view",
    "workspace",
    "working_copy",
    "trees",
    "tree_labels",
    "physical_workspace",
    "metadata_fingerprint",
    "physical_fingerprint",
  ]);
  if (
    pin.version !== 1 ||
    pin.repository !== repository ||
    pin.operation !== selection.operation ||
    workingTreeInteger(pin.binding_generation) !==
      workingTreeInteger(selection.bindingGeneration) ||
    !id(pin.repository, 32) ||
    !id(pin.operation, 128) ||
    !id(pin.view, 128) ||
    !id(pin.working_copy, 40) ||
    !text(pin.workspace, 256) ||
    !id(pin.physical_workspace, 64) ||
    !id(pin.metadata_fingerprint, 64) ||
    !id(pin.physical_fingerprint, 64)
  )
    fail();
  const trees = pin.trees,
    labels = pin.tree_labels;
  if (
    !Array.isArray(trees) ||
    !trees.length ||
    trees.length > 65 ||
    trees.length % 2 === 0 ||
    trees.some((tree) => !id(tree, 40)) ||
    !Array.isArray(labels) ||
    (labels.length && (trees.length === 1 || labels.length !== trees.length)) ||
    labels.some((label) => !text(label, 4096, true))
  )
    fail();
  const obstacles = workingTreeInteger(result.obstacle_count, 4096n);
  if (
    obstacles === null ||
    !Array.isArray(result.changes) ||
    result.changes.length > 4096
  )
    fail();
  let previous: string | undefined,
    beforeBytes = 0n,
    afterBytes = 0n,
    added = 0n;
  for (const value of result.changes) {
    const change = record(value, ["path", "status", "before", "after"]);
    const path = change.path;
    if (
      typeof path !== "string" ||
      !safePath(path) ||
      !text(path, 4096) ||
      path
        .split("/")
        .some((part) => [".git", ".jj"].includes(part.toLowerCase())) ||
      (previous !== undefined && compareUtf8(previous, path) >= 0)
    )
      fail();
    previous = path;
    const before = change.before === null ? null : file(change.before);
    const after = change.after === null ? null : file(change.after);
    if (change.status === "added" && !before && after) added++;
    else if (change.status === "deleted" && before && !after) {
      /* valid deletion */
    } else if (change.status === "modified" && before && after) {
      if (
        before.bytes === after.bytes &&
        before.digest === after.digest &&
        before.type === after.type &&
        before.executable === after.executable
      )
        fail();
    } else fail();
    beforeBytes += before?.bytes ?? 0n;
    afterBytes += after?.bytes ?? 0n;
    if (beforeBytes > 8n * 1024n * 1024n || afterBytes > 8n * 1024n * 1024n)
      fail();
  }
  if (added > obstacles) fail();
  return value as WorkingTreeObservation;
}
/** Rust sorts UTF-8 paths; JS UTF-16 ordering differs for non-BMP names. */
function compareUtf8(a: string, b: string) {
  const left = encoder.encode(a),
    right = encoder.encode(b);
  for (let i = 0; i < Math.min(left.length, right.length); i++)
    if (left[i] !== right[i]) return left[i]! - right[i]!;
  return left.length - right.length;
}
