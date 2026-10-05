import type { FileListingFile } from "@lapismd/design-core/diff";
export interface Bookmark {
  name: string;
  targets: readonly string[];
  default?: boolean;
  workspace?: boolean;
  description?: string;
  author?: string;
  authorEmail?: string;
  updatedAt?: number;
}
export interface RevisionDetail {
  files: FileListingFile[];
  patches: Readonly<Record<string, string>>;
  patchUnavailableReasons?: Readonly<
    Partial<
      Record<string, import("./repository-diff.js").DiffUnavailableReason>
    >
  >;
  pendingPatch?: string | undefined;
}
