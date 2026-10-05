import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";
import {
  conflictCommit,
  conflicts as conflictFixtures,
} from "../fixtures/conflicts.js";

export function createInMemoryStoreConflicts(route: () => StoreRoute) {
  const current = () =>
    route().commitId === conflictCommit ? [...conflictFixtures] : [];
  return {
    get view() {
      const conflicts = current();
      return {
        conflictCount: route().commitId ? conflicts.length : null,
        conflicts,
        conflictResolutionActions: ["select_term", "delete", "uploaded_file"],
        conflictUploadBytes: 262144,
      };
    },
    actions: {
      async uploadConflict(
        commit: string,
        path: string,
        fingerprint: string,
        bytes: Uint8Array,
      ) {
        if (
          commit !== route().commitId ||
          !current().some(
            (conflict) =>
              conflict.path === path && conflict.fingerprint === fingerprint,
          )
        )
          throw new Error("Conflict upload binding is stale.");
        if (bytes.byteLength > 262144)
          throw new Error("Conflict files are limited to 262144 bytes.");
        return {
          version: 1,
          id: "f1".repeat(32),
          file: "f2".repeat(64),
          bytes: bytes.byteLength,
          binding: {
            expected: {
              epoch: "f3".repeat(16),
              position: 1,
              heads: [route().snapshotOperationId!],
            },
            commit,
            path,
            fingerprint,
          },
        };
      },
    },
  };
}
