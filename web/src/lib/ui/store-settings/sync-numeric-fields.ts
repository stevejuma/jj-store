import {
  bytesUnits,
  countUnits,
  millisecondsUnits,
  secondsUnits,
} from "./numeric-setting.js";
export const syncNumericFields = {
  max_batch_origins: {
    label: "Origins per batch",
    units: countUnits("origins"),
  },
  max_closure_entries: {
    label: "Closure entries",
    units: countUnits("entries"),
  },
  max_objects_per_step: {
    label: "Objects per step",
    units: countUnits("objects"),
  },
  max_bytes_per_step: { label: "Transfer per step", units: bytesUnits },
  max_native_object_bytes: {
    label: "Maximum native object size",
    units: bytesUnits,
  },
  poll_millis: { label: "Polling interval", units: millisecondsUnits },
  retry_initial_millis: {
    label: "Initial retry delay",
    units: millisecondsUnits,
  },
  retry_max_seconds: { label: "Maximum retry delay", units: secondsUnits },
  circuit_failures: {
    label: "Failures before cooldown",
    units: countUnits("failures"),
  },
  circuit_cooldown_seconds: { label: "Cooldown", units: secondsUnits },
  timeout_seconds: { label: "Sync timeout", units: secondsUnits },
} as const;
