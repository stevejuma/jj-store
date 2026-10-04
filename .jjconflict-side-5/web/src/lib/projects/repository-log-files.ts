import type { FileListingFile } from "@lapismd/design-core/diff";

export function repositoryLogFileStats(
  files: readonly FileListingFile[],
  path: string | null | undefined,
): { additions: number; deletions: number } {
  const file = files.find((item) => item.path === path);
  return {
    additions: file?.additions ?? 0,
    deletions: file?.deletions ?? 0,
  };
}
