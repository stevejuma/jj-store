import {
  bytesUnits,
  countUnits,
  millisecondsUnits,
  secondsUnits,
} from "./numeric-setting.js";
export const searchNumericFields = {
  poll_millis: {
    label: "Polling interval",
    units: millisecondsUnits,
    description: "How often the worker checks for updates.",
  },
  timeout_seconds: {
    label: "Batch timeout",
    units: secondsUnits,
    description: "Maximum time to wait for a full batch.",
  },
  snapshot_every: {
    label: "Snapshot interval",
    units: countUnits("positions"),
    description: "Zero disables optional remote snapshots.",
  },
  max_text_bytes: {
    label: "Maximum text file size",
    units: bytesUnits,
    description: "Maximum text content extracted from each file.",
  },
  max_files: {
    label: "Files per extraction",
    units: countUnits("files"),
    description: "Maximum number of files extracted per batch.",
  },
  max_work: {
    label: "Extraction work",
    units: countUnits("units"),
    description: "Maximum work performed per extraction.",
  },
  max_output_bytes: {
    label: "Extraction output size",
    units: bytesUnits,
    description: "Maximum output produced per extraction.",
  },
} as const;
