import type { StoreConflictDescriptor } from "../../web/src/lib/controller/contracts.js";
import { IDS } from "./repository.js";

const file = (id: string) =>
  ({
    type: "file",
    id,
    executable: false,
    copy: { type: "placeholder" },
  }) as const;

export const conflictCommit = IDS.merge;
export const conflicts: readonly StoreConflictDescriptor[] = [
  {
    path: "src/lib.rs",
    fingerprint: "a1".repeat(64),
    terms: [
      { id: 0, role: "add", label: "native", value: file("b1".repeat(64)) },
      { id: 1, role: "remove", label: "base", value: file("b2".repeat(64)) },
      {
        id: 2,
        role: "add",
        label: "publication",
        value: file("b3".repeat(64)),
      },
    ],
    editor: {
      status: "text",
      contents: [
        "pub fn store() {\n    stream_native_objects();\n}\n",
        "pub fn store() {}\n",
        "pub fn store() {\n    preserve_publication_history();\n}\n",
      ],
      executable: false,
    },
  },
  {
    path: "assets/logo.bin",
    fingerprint: "c1".repeat(64),
    terms: [
      { id: 0, role: "add", label: "native", value: file("d1".repeat(64)) },
      { id: 1, role: "remove", label: "base", value: file("d2".repeat(64)) },
      {
        id: 2,
        role: "add",
        label: "publication",
        value: file("d3".repeat(64)),
      },
    ],
    editor: { status: "unsupported", reason: "binary" },
  },
];
