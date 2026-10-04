import type { CodeDiffChange } from "../controller/repository-read-types.js";

export type DiffUnavailableReason = Extract<
  CodeDiffChange["content"],
  { status: "unavailable" }
>["reason"];

export interface SelectedPatch {
  patch?: string;
  unavailableReason?: DiffUnavailableReason;
}

export function unavailableDiffMessage(reason?: DiffUnavailableReason): {
  title: string;
  description: string;
} {
  const descriptions: Record<DiffUnavailableReason, string> = {
    binary: "This file contains binary or invalid UTF-8 data.",
    complexity: "This text comparison exceeds the server's diff work limit.",
    too_large: "This file exceeds the server's text preview size limit.",
    conflict: "This comparison contains unresolved conflict terms.",
    multiple_sources: "This comparison has multiple source versions.",
    structural: "This change compares file types without a text diff.",
    not_requested: "Text content was not requested for this comparison.",
  };
  return {
    title:
      reason === "binary" ? "Binary diff unavailable" : "Text diff unavailable",
    description: reason
      ? descriptions[reason]
      : "No text diff is available for this comparison.",
  };
}
