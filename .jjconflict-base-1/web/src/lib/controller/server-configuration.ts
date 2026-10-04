import { decode, pretty, type Model, type Exact } from "../api/client.js";
import type { operations } from "../api/generated.js";
export type ServerSettings = Model<"LocalServerSettings">;
export type Configuration = Model<"ServerConfiguration">;
export type Section = keyof Configuration;
export type SyncEntry = Model<"SyncConfiguration">;
export type SaveServerSettings = Exact<
  NonNullable<
    operations["local_server_save_settings"]["requestBody"]
  >["content"]["application/json"]
>;
export const copy = <T>(value: T): T => decode(pretty(value)) as T;
export const equal = (a: unknown, b: unknown): boolean => {
  const sorted = (v: unknown): unknown =>
    Array.isArray(v)
      ? v.map(sorted)
      : v && typeof v === "object"
        ? Object.fromEntries(
            Object.entries(v)
              .sort(([a], [b]) => a.localeCompare(b))
              .map(([k, v]) => [k, sorted(v)]),
          )
        : v;
  return pretty(sorted(a)) === pretty(sorted(b));
};
export const serviceFields = [
  {
    key: "concurrent_requests",
    label: "Concurrent requests",
    min: 1,
    max: 256,
  },
  {
    key: "request_seconds",
    label: "Request timeout (seconds)",
    min: 1,
    max: 300,
  },
  {
    key: "max_blob_bytes",
    label: "Maximum blob size (bytes)",
    min: 1,
    max: 2147483648,
  },
  { key: "inventory_nodes", label: "Inventory nodes", min: 1, max: 1000000 },
] as const;
export const syncFields = [
  { key: "max_batch_origins", label: "Origins per batch", min: 1, max: 128 },
  {
    key: "max_closure_entries",
    label: "Closure entries",
    min: 1,
    max: 1000000,
  },
  {
    key: "max_objects_per_step",
    label: "Objects per step",
    min: 1,
    max: 1000000,
  },
  {
    key: "max_bytes_per_step",
    label: "Bytes per step",
    min: 1,
    max: 2147483648,
  },
  {
    key: "max_native_object_bytes",
    label: "Native object bytes",
    min: 113,
    max: 5368709120,
  },
  { key: "poll_millis", label: "Polling interval (ms)", min: 10, max: 60000 },
  {
    key: "retry_initial_millis",
    label: "Initial retry delay (ms)",
    min: 1,
    max: 60000,
  },
  {
    key: "retry_max_seconds",
    label: "Maximum retry delay (seconds)",
    min: 1,
    max: 3600,
  },
  {
    key: "circuit_failures",
    label: "Failures before cooldown",
    min: 1,
    max: 1024,
  },
  {
    key: "circuit_cooldown_seconds",
    label: "Cooldown (seconds)",
    min: 1,
    max: 3600,
  },
  { key: "timeout_seconds", label: "Sync timeout (seconds)", min: 1, max: 300 },
] as const;
export function validateConfiguration(value: Configuration) {
  for (const field of serviceFields) number(value.service[field.key], field);
  for (const names of [value.protection.bookmarks, value.protection.commits]) {
    if (!Array.isArray(names)) throw new Error("Check the protection rules.");
  }
  if (
    value.protection.bookmarks.length > 1024 ||
    value.protection.commits.length > 100000 ||
    value.protection.bookmarks.some(
      (n) => !n || n.length > 1024 || /\p{Cc}/u.test(n),
    ) ||
    value.protection.commits.some((id) => !/^[0-9a-f]{128}$/.test(id))
  )
    throw new Error(
      "Use bookmark names and full 128-character commit IDs, one per line.",
    );
  if (
    value.sync.length > 8 ||
    new Set(value.sync.map((e) => e.repository)).size !== value.sync.length ||
    new Set(value.sync.map((e) => e.token_environment)).size !==
      value.sync.length
  )
    throw new Error(
      "Use at most eight unique repositories and credential references.",
    );
  for (const entry of value.sync) {
    if (
      !/^[0-9a-f]{32}$/.test(entry.repository) ||
      !/^JJ_STORE_REPLICATION_TOKEN_[A-Z0-9_]+$/.test(
        entry.token_environment,
      ) ||
      entry.token_environment.length > 128
    )
      throw new Error(
        "Select a repository and a JJ_STORE_REPLICATION_TOKEN_ environment name.",
      );
    const url = new URL(entry.upstream);
    const loopback = /^127\./.test(url.hostname) || url.hostname === "[::1]";
    if (
      url.username ||
      url.password ||
      url.search ||
      url.hash ||
      url.pathname !== "/" ||
      !(
        url.protocol === "https:" ||
        (url.protocol === "http:" && entry.insecure_loopback && loopback)
      )
    )
      throw new Error(
        "Use an HTTPS origin, or explicitly enabled literal loopback HTTP.",
      );
    if (
      entry.ca_certificate &&
      (!entry.ca_certificate.startsWith("/") ||
        /\p{Cc}/u.test(entry.ca_certificate))
    )
      throw new Error("CA certificate must be an absolute file path.");
    for (const field of syncFields) number(entry.limits[field.key], field);
    if (
      Number(entry.limits.retry_initial_millis) >
      Number(entry.limits.retry_max_seconds) * 1000
    )
      throw new Error("Initial retry delay must not exceed the maximum delay.");
  }
}
function number(
  value: unknown,
  field: { label: string; min: number; max: number },
) {
  const n = Number(value);
  if (!Number.isSafeInteger(n) || n < field.min || n > field.max)
    throw new Error(
      `${field.label} must be a whole number from ${field.min} to ${field.max}.`,
    );
}
export function validateSettings(value: ServerSettings) {
  if (
    value.version !== 1 ||
    !/^[0-9a-f]{64}$/.test(value.revision) ||
    !Array.isArray(value.inventory) ||
    value.inventory.length > 256
  )
    throw new Error("Unsupported server settings response.");
  validateConfiguration(value.desired);
  validateConfiguration(value.effective);
}
