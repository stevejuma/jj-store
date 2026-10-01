import type { SearchSettings } from "../../web/src/lib/controller/search-settings.js";
export function searchSettingsValue(): NonNullable<SearchSettings["value"]> {
  const config = {
    enabled: true,
    index_source: true,
    scopes: { main: { kind: "bookmark" as const, name: "main" } },
    include_prefixes: [],
    exclude_prefixes: [],
    index_bytes: 536870912,
  };
  return {
    version: 1,
    revision: "ab".repeat(32),
    desired: config,
    effective: config,
    operation: "idle",
    error: null,
  };
}
