import type { Snapshot, Head } from "./client.js";

export class ApiError extends Error {
  constructor(
    readonly code: string,
    readonly snapshot?: Snapshot,
    readonly heads?: Head,
    readonly diagnostic?: { kind: string; message: string },
  ) {
    super(code.replaceAll("_", " "));
  }
}
