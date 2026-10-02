import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex } from "@noble/hashes/utils.js";
import type { StoreRoute } from "../controller/contracts.js";
export const disclosureStorageKey = "jj-store/layout/v1/disclosures";
export function disclosureScope(route: StoreRoute): readonly string[] {
  return route.page === "projects"
    ? [
        "projects",
        route.workHome ?? "",
        route.projectId ?? "",
        route.workRepository ?? "",
      ]
    : ["repository", route.repository];
}
export function disclosureKey(
  route: StoreRoute,
  section: readonly string[],
): string {
  return bytesToHex(
    sha256(
      new TextEncoder().encode(
        JSON.stringify([...disclosureScope(route), ...section]),
      ),
    ),
  );
}
export function loadDisclosures(storage?: Storage): Record<string, boolean> {
  try {
    const value = JSON.parse(storage?.getItem(disclosureStorageKey) ?? "null");
    if (
      value?.version === 1 &&
      value.flags &&
      typeof value.flags === "object" &&
      !Array.isArray(value.flags) &&
      Object.entries(value.flags).length <= 1024 &&
      Object.entries(value.flags).every(
        ([key, flag]) =>
          /^[a-f0-9]{64}$/.test(key) && typeof flag === "boolean",
      )
    )
      return value.flags;
  } catch {
    /* Layout storage cannot block browsing. */
  }
  return {};
}
export function saveDisclosures(
  storage: Storage | undefined,
  flags: Record<string, boolean>,
) {
  try {
    storage?.setItem(
      disclosureStorageKey,
      JSON.stringify({ version: 1, flags }),
    );
  } catch {
    /* Layout storage cannot block browsing. */
  }
}
