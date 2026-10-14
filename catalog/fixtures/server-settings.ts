import type { ServerSettings } from "../../web/src/lib/controller/server-configuration.js";
export function serverSettingsValue(): ServerSettings {
  const desired = {
    service: {
      concurrent_requests: 8,
      request_seconds: 30,
      max_blob_bytes: 33554432,
      inventory_nodes: 100000,
    },
    protection: { bookmarks: ["main"], commits: [] },
    sync: [],
    workflow: { admitted: [], local: null },
  };
  return {
    version: 1,
    revision: "ab".repeat(32),
    desired: structuredClone(desired),
    effective: structuredClone(desired),
    restart_required: false,
    workflow_runtime: {
      principal: "02".repeat(16),
      available: true,
      running: false,
      busy: false,
      message: null,
    },
    sync_defaults: {
      max_batch_origins: 128,
      max_closure_entries: 100000,
      max_objects_per_step: 100000,
      max_bytes_per_step: 1073741824,
      max_native_object_bytes: 67108864,
      poll_millis: 5000,
      retry_initial_millis: 500,
      retry_max_seconds: 60,
      circuit_failures: 8,
      circuit_cooldown_seconds: 60,
      timeout_seconds: 30,
    },
    inventory: [
      ...[
        ["service", "state_dir", "/private/example/local"],
        [
          "service",
          "access.catalog",
          "/private/example/local/repositories.json",
        ],
        ["service", "access.mode", "local-trusted"],
        ["service", "tls", "Not configured"],
        ["storage", "storage.mode", "embedded-rustfs"],
        [
          "storage",
          "storage.credentials",
          "/private/example/credential-reference.json",
        ],
        ["storage", "backup", "None"],
      ].map(([section, name, saved]) => ({
        section,
        name,
        saved,
        effective: saved,
        reason: "Managed by setup.",
      })),
      {
        section: "service",
        name: "bind",
        saved: "127.0.0.1:7472",
        effective: "127.0.0.1:7472",
        reason: "Managed by server or desktop launch.",
      },
      {
        section: "storage",
        name: "storage.volume",
        saved: "/private/example/rustfs",
        effective: "/private/example/rustfs",
        reason: "Authority storage changes require migration.",
      },
    ],
  };
}
