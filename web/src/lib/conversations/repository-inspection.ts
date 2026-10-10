import { assertPinned } from "../controller/production-snapshot.js";
import { repositoryComparison } from "./repository-comparison.js";
import type { StoreDataSource } from "../controller/native-data-source.js";
import type { CodeEntry } from "../controller/repository-read-types.js";
import type { WorkspaceRead } from "./workspace-browser.svelte.js";

/** Read-only presentation of a pinned repository. This never creates a session or workspace source. */
export class RepositoryInspection {
  private pins = new Map<
    string,
    Promise<{ operation: string; commit: string }>
  >();
  private entries = new Map<string, Map<string, CodeEntry>>();
  constructor(private readonly acquire: (id: string) => StoreDataSource) {}
  async read(
    repository: string,
    path: string | null,
    cursor: string | null,
    signal?: AbortSignal,
    comparison?: {
      range: import("../api/client.js").Model<"ComparisonRange">;
      base: string | null;
    },
  ): Promise<WorkspaceRead> {
    const source = this.acquire(repository);
    try {
      let pin = this.pins.get(repository);
      if (!pin) {
        pin = (async () => {
          const head = await source.snapshot();
          if (head.heads.length !== 1)
            throw new Error(
              "Choose one repository operation before browsing its files.",
            );
          const operation = head.heads[0]!;
          const summary = await source.codeSummary!(operation, signal);
          assertPinned(summary.snapshot, source, operation);
          if (!summary.data.latest)
            throw new Error("This repository has no committed files.");
          return { operation, commit: summary.data.latest.id };
        })();
        this.pins.set(repository, pin);
        void pin.catch(() => {
          if (this.pins.get(repository) === pin) this.pins.delete(repository);
        });
      }
      const { operation, commit } = await pin;
      if (comparison)
        return {
          version: 1,
          session: `repository:${repository}`,
          repository,
          workspace: "Repository snapshot",
          location: "",
          operation,
          commit,
          ...(await repositoryComparison(
            source,
            operation,
            commit,
            path,
            cursor,
            comparison.range,
            comparison.base,
            signal,
          )),
        };
      let entries = this.entries.get(repository);
      if (!entries) {
        entries = new Map();
        this.entries.set(repository, entries);
      }
      let next: string | null = null;
      let listing: CodeEntry[] = [];
      if (!path) {
        const page = await source.paths(commit, operation, {
          path: "",
          recursive: true,
          includeLastChange: false,
          cursor: cursor ?? undefined,
          limit: 256,
          signal,
        });
        assertPinned(page.snapshot, source, operation);
        listing = page.data.entries;
        for (const entry of listing) entries.set(entry.path, entry);
        next = page.next_cursor;
      } else if (!entries.has(path)) {
        const directory = path.includes("/")
          ? path.slice(0, path.lastIndexOf("/"))
          : "";
        let after: string | undefined;
        for (let pageNumber = 0; pageNumber < 20; pageNumber++) {
          const page = await source.paths(commit, operation, {
            path: directory,
            recursive: false,
            includeLastChange: false,
            cursor: after,
            limit: 256,
            signal,
          });
          assertPinned(page.snapshot, source, operation);
          for (const entry of page.data.entries) entries.set(entry.path, entry);
          if (entries.has(path) || !page.next_cursor) break;
          after = page.next_cursor;
        }
      }
      const entry = path ? entries.get(path) : undefined;
      let file: WorkspaceRead["file"] = null;
      if (path) {
        const term = entry?.terms.length === 1 ? entry.terms[0] : null;
        if (!term || term.type !== "file")
          file = {
            path,
            fingerprint: commit,
            contents: null,
            before: null,
            unavailable:
              "This path is unavailable as text in the pinned repository.",
          };
        else {
          const value = await source.file(term.id, operation, {
            commit,
            path,
            signal,
          });
          file = {
            path,
            fingerprint: term.id,
            contents: value.text ?? null,
            before: null,
            unavailable:
              value.text === undefined
                ? "This file is binary or exceeds the reading limit."
                : null,
          };
        }
      }
      return {
        version: 1,
        session: `repository:${repository}`,
        repository,
        workspace: "Repository snapshot",
        location: "",
        operation,
        commit,
        selection: `repository:${repository}:${operation}:${commit}`,
        references: [],
        comparison: null,
        incomplete: null,
        files: path
          ? []
          : listing.map((e) => ({
              path: e.path,
              kind: e.conflicted
                ? "conflict"
                : (e.terms[0]?.type ?? "unavailable"),
              bytes: 0,
              fingerprint: commit,
              status: null,
            })),
        next,
        file,
      };
    } finally {
      source.close();
    }
  }
}
