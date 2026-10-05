import { encodeJjChangeId } from "../../web/src/lib/projects/repository-log-graph.js";
import { IDS, revisions, bookmarks } from "./repository.js";
import type {
  RepositoryChoice,
  RepositorySnapshot,
} from "../../web/src/lib/controller/contracts.js";
export const repositories = [
  { id: "jj-store", name: "jj-store", permission: "admin" as const },
  { id: "native-demo", name: "native-demo", permission: "write" as const },
];
export function repositoryChoices(
  sourceBookmarks = bookmarks,
  sourceRevisions = revisions,
): readonly RepositoryChoice[] {
  return [
    ...sourceBookmarks.map((bookmark) => ({
      value: `bookmark:${bookmark.name}`,
      label: bookmark.name,
      kind: "bookmark" as const,
      targets: bookmark.targets,
    })),
    ...sourceRevisions.map((revision) => ({
      value: `commit:${revision.eventId}`,
      label: `${encodeJjChangeId(revision.changeId ?? "").slice(0, 8)} · ${revision.eventId.slice(0, 8)} · ${revision.message}`,
      kind: "change" as const,
      searchTerms: [
        revision.changeId ?? "",
        encodeJjChangeId(revision.changeId ?? ""),
      ],
      targets: [revision.eventId],
    })),
  ];
}
export const choices: readonly RepositoryChoice[] = repositoryChoices();
export function repositorySnapshot(
  repository: string,
  id: string,
): RepositorySnapshot {
  const files = [
    {
      path: "README.md",
      contents: `# ${repository}\n\nNative JJ snapshot ${id}\n\n<script>alert('source')</script>\n`,
    },
    {
      path: "Cargo.toml",
      contents: `[package]\nname = "${repository}"\nversion = "0.1.0"\nedition = "2024"\n`,
    },
  ];
  if (id !== IDS.root)
    files.push({
      path: "src/lib.rs",
      contents:
        id === IDS.left
          ? "pub fn store() {\n    stream_native_objects();\n}\n"
          : id === IDS.right
            ? "pub fn store() {\n    preserve_publication_history();\n}\n"
            : "pub fn store() {\n    stream_native_objects();\n    preserve_publication_history();\n}\n",
    });
  return { id, files: [...files, { path: "assets/logo.bin", binary: true }] };
}
