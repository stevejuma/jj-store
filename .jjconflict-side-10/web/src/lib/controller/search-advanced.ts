export const searchAdvancedFields = [
  {
    key: "poll_millis",
    label: "Polling interval (ms)",
    min: 10,
    max: 60000,
    default: 1000,
  },
  {
    key: "timeout_seconds",
    label: "Batch timeout (seconds)",
    min: 1,
    max: 300,
    default: 60,
  },
  {
    key: "snapshot_every",
    label: "Snapshot interval (positions)",
    min: 0,
    max: 1000000,
    default: 32,
  },
  {
    key: "max_text_bytes",
    label: "Maximum text file bytes",
    min: 1,
    max: 1048576,
    default: 1048576,
  },
  {
    key: "max_files",
    label: "Files per extraction",
    min: 1,
    max: 10000,
    default: 10000,
  },
  {
    key: "max_work",
    label: "Extraction work units",
    min: 1,
    max: 1000000,
    default: 1000000,
  },
  {
    key: "max_output_bytes",
    label: "Extraction output bytes",
    min: 1,
    max: 268435456,
    default: 67108864,
  },
] as const;
export type SearchAdvancedKey = (typeof searchAdvancedFields)[number]["key"];
export const searchAdvancedDefaults = Object.fromEntries(
  searchAdvancedFields.map((f) => [f.key, f.default]),
) as Record<SearchAdvancedKey, number>;
