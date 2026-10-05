import type { Snapshot, Head, Model } from "./client.js";

export type ErrorSnapshot =
  | Snapshot
  | (Model<"LocalCodeSnapshot"> & {
      profile: "git_sha1_jj045";
      heads: string[];
    });

export class ApiError extends Error {
  constructor(
    readonly code: string,
    readonly snapshot?: ErrorSnapshot,
    readonly heads?: Head,
    readonly diagnostic?: { kind: string; message: string },
    readonly localSnapshot?: Model<"LocalCodeSnapshot">,
  ) {
    super(code.replaceAll("_", " "));
  }
}
