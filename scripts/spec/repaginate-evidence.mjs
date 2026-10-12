import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { mergeEvidencePages, evidencePages } from "./evidence-format.mjs";
import { signature } from "./evidence-signature.mjs";
import { writeEvidence } from "./evidence-new.mjs";

/** Change only page representation; fail before writing if payloads differ. */
export function repaginateEvidence(directory, root = ".") {
  const pages = () =>
    readdirSync(join(root, directory))
      .filter((name) => /^(?:index|details-\d+)\.md$/.test(name))
      .sort((a, b) =>
        a === "index.md" ? -1 : b === "index.md" ? 1 : a.localeCompare(b),
      )
      .map((name) => readFileSync(join(root, directory, name), "utf8"));
  const original = mergeEvidencePages(pages());
  if (
    signature(original) !==
    signature(mergeEvidencePages(evidencePages(original)))
  )
    throw new Error(`evidence payload cannot be preserved for ${original.id}`);
  writeEvidence(original, root);
  if (signature(original) !== signature(mergeEvidencePages(pages())))
    throw new Error(
      `evidence payload changed while repaginating ${original.id}`,
    );
  return original.id;
}
if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
)
  console.log(
    `Repaginated ${repaginateEvidence(process.argv[2])} without changing payload.`,
  );
