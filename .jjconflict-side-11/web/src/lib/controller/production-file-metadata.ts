import { pretty } from "../api/client.js";
import { createFileReadQueue } from "./file-read-queue.js";
import {
  readLandingDocuments,
  readPaths,
  readFileRevision,
} from "./production-store-loaders.js";
import { landingDocuments } from "./production-landing-documents.js";
import { pathPageKey } from "./production-file-pages.js";
import type { FileReaderContext } from "./production-file-reader.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";

export function createProductionFileMetadata(
  context: FileReaderContext,
  publish: (commit: string) => void,
) {
  const { data } = context;
  const queue = createFileReadQueue();
  let selected = "";
  let source: StoreDataSource | undefined;
  let scope = new AbortController();
  let signal = scope.signal;
  function bind(commit: string, operation: string) {
    const key = `${context.version()}:${operation}:${commit}`;
    if (key !== selected || source !== context.source()) {
      scope.abort();
      queue.clear();
      scope = new AbortController();
      const parent = context.signal?.();
      signal = parent ? AbortSignal.any([scope.signal, parent]) : scope.signal;
      signal.addEventListener("abort", () => queue.clear(), { once: true });
      selected = key;
      source = context.source();
    }
    return signal;
  }
  function load(input: {
    source: StoreDataSource;
    operation: string;
    commit: string;
    ticket: number;
    value: Awaited<ReturnType<typeof readPaths>>;
    anchor?: string;
  }) {
    const { source, operation, commit, ticket, value, anchor } = input;
    const readSignal = signal;
    const selectedCurrent = () =>
      context.current(ticket, source) && data.snapshot?.id === commit;
    const current = () => !readSignal.aborted && selectedCurrent();
    const add = (key: string, run: () => Promise<void>, failed: () => void) =>
      queue.add(
        key,
        async () => {
          if (!current()) return;
          try {
            await run();
          } catch {
            if (current()) failed();
          }
        },
        () => {
          if (selectedCurrent()) failed();
        },
      );
    if (data.fileRevision?.eventId !== commit) {
      data.fileRevisionStatus = "loading";
      add(
        `header:${commit}`,
        async () => {
          const revision = await readFileRevision(
            source,
            operation,
            commit,
            readSignal,
          );
          if (current()) {
            data.fileRevision = revision;
            data.fileRevisionStatus = "ready";
          }
        },
        () => {
          data.fileRevisionStatus = "unavailable";
        },
      );
    }
    const deferred = value.entries.some(
      (entry) =>
        entry.last_change.status === "unavailable" &&
        entry.last_change.reason === "not_requested",
    );
    if (deferred) {
      const key = pathPageKey(value.path, value.recursive);
      const markUnavailable = () => {
        const page = data.pathPages.get(key);
        if (!page) return;
        const paths = new Set(value.entries.map((entry) => entry.path));
        data.pathPages = new Map(data.pathPages).set(key, {
          ...page,
          unavailableChanges: new Set([
            ...(page.unavailableChanges ?? []),
            ...paths,
          ]),
        });
        publish(commit);
      };
      add(
        `attribution:${key}:${anchor ?? ""}`,
        async () => {
          const result = await readPaths(source, operation, commit, {
            path: value.path,
            recursive: value.recursive,
            includeLastChange: true,
            anchor,
            limit: value.entries.length,
            expected: value.readSnapshot,
            signal: readSignal,
          });
          if (!current()) return;
          const page = data.pathPages.get(key);
          if (!page) return;
          const records = new Map(
            result.entries.map((entry) => [entry.path, entry]),
          );
          if (
            value.entries.some(
              (entry) =>
                pretty(records.get(entry.path)?.terms) !== pretty(entry.terms),
            )
          )
            throw new Error("File attribution selection changed.");
          data.pathPages = new Map(data.pathPages).set(key, {
            ...page,
            unavailableChanges: new Set(
              [...(page.unavailableChanges ?? [])].filter(
                (path) => !records.has(path),
              ),
            ),
            entries: page.entries.map((entry) => {
              const record = records.get(entry.path);
              return record && pretty(record.terms) === pretty(entry.terms)
                ? record
                : entry;
            }),
          });
          publish(commit);
        },
        markUnavailable,
      );
    }
    add(
      `documents:${commit}:${landingDocuments(data.fileTerms)
        .map(({ path, id }) => `${path}:${id}`)
        .join(":")}`,
      async () => {
        const snapshot = data.snapshot;
        if (!snapshot) return;
        const files = new Map(data.fileTerms);
        const enriched = await readLandingDocuments(
          source,
          operation,
          snapshot,
          files,
          readSignal,
        );
        if (!current() || !data.snapshot) return;
        const loaded = new Map(
          enriched.files
            .filter((file) => file.contents !== undefined || file.binary)
            .map((file) => [file.path, file]),
        );
        data.snapshot = {
          ...data.snapshot,
          files: data.snapshot.files.map((file) =>
            files.get(file.path) === data.fileTerms.get(file.path)
              ? (loaded.get(file.path) ?? file)
              : file,
          ),
        };
      },
      () => {},
    );
  }
  return { bind, load };
}
