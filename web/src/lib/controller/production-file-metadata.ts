import { ApiError, pretty } from "../api/client.js";
import { createFileReadQueue } from "./file-read-queue.js";
import {
  readLandingDocuments,
  readPaths,
  readFileRevision,
} from "./production-store-loaders.js";
import { landingDocuments } from "./production-landing-documents.js";
import {
  pathPageKey,
  type PathMetadataWindow,
} from "./production-file-pages.js";
import type { FileReaderContext } from "./production-file-reader.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";

export function createProductionFileMetadata(
  context: FileReaderContext,
  publish: (commit: string) => void,
) {
  const { data } = context;
  const queue = createFileReadQueue();
  const scheduled = new WeakMap<PathMetadataWindow, AbortSignal>();
  let selected = "";
  let source: StoreDataSource | undefined;
  let scope = new AbortController();
  let signal = scope.signal;
  function bind(commit: string, operation: string, directory: string) {
    const key = `${context.version()}:${operation}:${commit}:${directory}`;
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
    window?: PathMetadataWindow;
  }) {
    const { source, operation, commit, ticket, value, anchor, window } = input;
    const readSignal = signal;
    const selectedCurrent = () =>
      context.current(ticket, source) && data.snapshot?.id === commit;
    const current = () => !readSignal.aborted && selectedCurrent();
    const add = (
      key: string,
      run: () => Promise<void>,
      failed: (cause?: unknown) => void,
    ) =>
      queue.add(
        key,
        async () => {
          if (!current()) return;
          try {
            await run();
          } catch (cause) {
            if (current()) failed(cause);
          }
        },
        () => {
          if (selectedCurrent()) failed();
        },
      );
    const deferred = value.entries.some(
      (entry) =>
        entry.last_change.status === "unavailable" &&
        ["not_requested", "pending"].includes(entry.last_change.reason),
    );
    if (
      window?.phase !== "complete" &&
      (deferred || window?.phase === "history") &&
      (!window || scheduled.get(window) !== readSignal)
    ) {
      if (window) scheduled.set(window, readSignal);
      const key = pathPageKey(value.path, value.recursive);
      const retained = data.pathPages.get(key);
      if (retained?.unavailableChanges?.size) {
        const pending = new Set(value.entries.map((entry) => entry.path));
        data.pathPages = new Map(data.pathPages).set(key, {
          ...retained,
          unavailableChanges: new Set(
            [...retained.unavailableChanges].filter(
              (path) => !pending.has(path),
            ),
          ),
        });
        publish(commit);
      }
      const markUnavailable = (cause?: unknown) => {
        if (cause !== undefined && window) window.phase = "complete";
        const page = data.pathPages.get(key);
        if (!page) return;
        const paths = new Set(
          value.entries
            .filter(
              (entry) =>
                page.entries.find((current) => current.path === entry.path)
                  ?.last_change.status !== "available",
            )
            .map((entry) => entry.path),
        );
        data.pathPages = new Map(data.pathPages).set(key, {
          ...page,
          unavailableChanges: new Set([
            ...(page.unavailableChanges ?? []),
            ...paths,
          ]),
        });
        publish(commit);
        if (cause !== undefined) context.report(cause);
      };
      const batch = async (
        mode: "quick" | "history",
        pass: number,
      ): Promise<void> => {
        let attempts = 0;
        while (current()) {
          const result = await readPaths(source, operation, commit, {
            path: value.path,
            recursive: value.recursive,
            includeLastChange: true,
            ...(mode === "history" ? { lastChangeMode: mode } : {}),
            anchor,
            limit: value.entries.length,
            expected: value.readSnapshot,
            signal: readSignal,
          }).catch((error: unknown) => {
            if (error instanceof ApiError && error.code === "busy")
              return undefined;
            throw error;
          });
          if (!current()) return;
          if (!result) {
            await pause(
              Math.min(150 * 2 ** Math.min(attempts++, 3), 1000),
              readSignal,
            );
            continue;
          }
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
              if (!record || pretty(record.terms) !== pretty(entry.terms))
                return entry;
              if (
                entry.last_change.status === "available" &&
                record.last_change.status === "unavailable"
              )
                return entry;
              // Keep quick values visible while deeper history is pending.
              // Old servers without provenance complete after the quick read.
              return record.last_change.status === "unavailable" &&
                record.last_change.reason === "pending"
                ? {
                    ...record,
                    last_change: { status: "unavailable", reason: "budget" },
                  }
                : record;
            }),
          });
          publish(commit);
          const continueHistory =
            mode === "quick"
              ? result.entries.some(
                  (entry) =>
                    entry.last_change.status === "available" &&
                    entry.last_change.attribution !== undefined,
                )
              : result.entries.some(
                  (entry) =>
                    entry.last_change.status === "unavailable" &&
                    entry.last_change.reason === "pending",
                );
          if (window) window.phase = continueHistory ? "history" : "complete";
          if (continueHistory) {
            await pause(
              Math.min(150 * 2 ** Math.min(pass, 3), 1000),
              readSignal,
            );
            if (current())
              add(
                `history:${key}:${anchor ?? ""}:${pass}`,
                () => batch("history", pass + 1),
                markUnavailable,
              );
          }
          return;
        }
      };
      add(
        `attribution:${key}:${anchor ?? ""}`,
        () => batch(window?.phase === "history" ? "history" : "quick", 0),
        markUnavailable,
      );
    }
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

function pause(millis: number, signal: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    const aborted = () => {
      clearTimeout(timer);
      signal.removeEventListener("abort", aborted);
      reject(signal.reason);
    };
    const timer = setTimeout(() => {
      signal.removeEventListener("abort", aborted);
      resolve();
    }, millis);
    signal.addEventListener("abort", aborted, { once: true });
    if (signal.aborted) aborted();
  });
}
