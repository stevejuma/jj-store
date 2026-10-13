import type {
  ReloadCommand,
  ReloadOutcome,
  ReloadStatus,
} from "./runtime-reload.svelte.js";
const domains = new Set([
  "workflows",
  "service",
  "protection",
  "sync",
  "capture",
  "remote_agents",
  "search",
  "agents",
  "compute",
]);
const states = new Set([
  "running",
  "applied",
  "current",
  "deferred",
  "failed",
  "unknown",
]);
const id = (value: unknown, length: number) =>
  typeof value === "string" && new RegExp(`^[0-9a-f]{${length}}$`).test(value);
function domainList(value: unknown): value is ReloadCommand["domains"] {
  return (
    Array.isArray(value) &&
    value.length <= 8 &&
    new Set(value).size === value.length &&
    value.every((v) => domains.has(v))
  );
}
export function validateReloadCommand(value: ReloadCommand) {
  if (
    !value ||
    value.version !== 1 ||
    !id(value.request_id, 32) ||
    !id(value.expected_revision, 64) ||
    !domainList(value.domains) ||
    !value.domains.length
  )
    throw Error("Invalid reload intent");
}
export function validateReloadStatus(value: ReloadStatus) {
  if (
    !value ||
    !id(value.saved_revision, 64) ||
    !domainList(value.supported) ||
    !domainList(value.pending) ||
    value.pending.some((d) => !value.supported.includes(d)) ||
    !Number.isSafeInteger(value.generation) ||
    value.generation < 0
  )
    throw Error("Invalid reload status");
  if (value.original) {
    validateReloadCommand(value.original);
    if (value.original.domains.some((d) => !value.supported.includes(d)))
      throw Error("Unauthorized reload intent");
    if (value.outcome) validateReloadOutcome(value.outcome, value.original);
  }
}
export function validateReloadOutcome(
  value: ReloadOutcome,
  command: ReloadCommand,
) {
  if (
    !value ||
    value.request_id !== command.request_id ||
    !states.has(value.state) ||
    !Number.isSafeInteger(value.generation) ||
    value.generation < 1 ||
    !Array.isArray(value.results) ||
    value.results.length !== command.domains.length ||
    !domainList(value.results.map((r) => r.domain)) ||
    value.results.some(
      (r) =>
        !command.domains.includes(r.domain) ||
        !states.has(r.state) ||
        (r.reason !== null &&
          (typeof r.reason !== "string" || r.reason.length > 2048)),
    )
  )
    throw Error("Invalid original reload outcome");
}
