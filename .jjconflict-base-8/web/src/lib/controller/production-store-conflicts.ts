import { isLocal, type RepositoryHead } from "./repository-profile.js";
import type { Head, Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";

export function createProductionStoreConflicts(options: {
  source: () => StoreDataSource;
  head: () => RepositoryHead | null;
  current: (source: StoreDataSource) => boolean;
  uploadLimit: () => number | bigint;
}) {
  async function upload(
    commit: string,
    path: string,
    fingerprint: string,
    bytes: Uint8Array,
  ): Promise<Model<"ConflictUpload">> {
    const limit = options.uploadLimit();
    if (BigInt(bytes.byteLength) > BigInt(limit))
      throw new Error(`Conflict files are limited to ${limit} bytes.`);
    const head = options.head();
    if (!head || isLocal(head) || head.heads.length !== 1)
      throw new Error("Choose one current operation head before uploading.");
    const source = options.source();
    const result = await source.uploadConflict(
      head,
      commit,
      path,
      fingerprint,
      bytes,
    );
    if (!options.current(source))
      throw new DOMException("Session changed", "AbortError");
    return result;
  }
  return { upload };
}
