import { snapshotOperation } from "./repository-profile.js";
import type { StoreRoute } from "./contracts.js";
import type { ReaderContext } from "./production-store-reader.svelte.js";
import { readReferences } from "./production-store-loaders.js";
import { readWorkingCopies } from "./store-workspaces.js";

const unique = <T>(items: T[], key: (item: T) => string) => [
  ...new Map(items.map((item) => [key(item), item])).values(),
];

export function createProductionReferenceReader(context: ReaderContext) {
  const { data, history } = context;
  const pending = new Map<
    string,
    {
      ticket: number;
      source: ReturnType<ReaderContext["source"]>;
      work: Promise<void>;
    }
  >();
  function single(kind: string, run: () => Promise<void>) {
    const ticket = context.version(),
      source = context.source();
    const existing = pending.get(kind);
    if (existing?.ticket === ticket && existing.source === source)
      return existing.work;
    const work = run().finally(() => {
      if (pending.get(kind)?.work === work) pending.delete(kind);
    });
    pending.set(kind, { ticket, source, work });
    return work;
  }
  function loadReferences(more = false, optional = false) {
    return single("bookmarks", async () => {
      const source = context.source(),
        operation = context.operation(),
        ticket = context.version();
      if (!operation || (more && !data.refsCursor)) return;
      data.status.bookmarks = more ? "refreshing" : "loading";
      try {
        const value = await readReferences(
          source,
          operation,
          more ? (data.refsCursor ?? undefined) : undefined,
          more ? (data.refsPageSnapshot ?? undefined) : undefined,
          { kind: "bookmarks", signal: context.signal?.() },
        );
        if (!context.current(ticket, source)) return;
        const selected = history.route;
        data.bookmarks = unique(
          [
            ...(more
              ? data.bookmarks
              : data.bookmarks.filter(
                  (item) => item.name === selected.bookmark,
                )),
            ...value.bookmarks,
          ],
          (item) => item.name,
        );
        data.choices = unique(
          [
            ...(more
              ? data.choices
              : data.choices.filter(
                  (item) =>
                    item.value === `bookmark:${selected.bookmark}` ||
                    item.value === `change:${selected.changeId}`,
                )),
            ...value.choices,
          ],
          (item) => item.value,
        );
        data.refsCursor = value.cursor;
        data.refsPageSnapshot = value.snapshot;
        data.status.bookmarks = data.bookmarks.length ? "ready" : "empty";
      } catch (cause) {
        if (!context.current(ticket, source)) return;
        if (!optional) context.report(cause);
        data.status.bookmarks = "failed";
      }
    });
  }
  function loadWorkingCopies(more = false, optional = false) {
    return single("workspaces", async () => {
      const source = context.source(),
        operation = context.operation(),
        ticket = context.version();
      if (!operation || (more && !data.workingCopiesCursor)) return;
      data.workingCopiesStatus = more ? "refreshing" : "loading";
      try {
        const value = await readWorkingCopies(
          source,
          operation,
          more ? (data.workingCopiesCursor ?? undefined) : undefined,
          more ? (data.workingCopiesSnapshot ?? undefined) : undefined,
          context.signal?.(),
        );
        if (!context.current(ticket, source)) return;
        data.workingCopies = unique(
          [
            ...(more
              ? data.workingCopies
              : data.workingCopies.filter(
                  (item) => item.workspace === history.route.workspace,
                )),
            ...value.workingCopies,
          ],
          (item) => item.workspace,
        );
        data.workingCopiesCursor = value.cursor;
        data.workingCopiesSnapshot = value.snapshot;
        data.workingCopiesStatus = data.workingCopies.length
          ? "ready"
          : "empty";
      } catch (cause) {
        if (!context.current(ticket, source)) return;
        if (!optional) context.report(cause);
        data.workingCopiesStatus = "failed";
      }
    });
  }
  async function prepareReferences(route: StoreRoute) {
    const source = context.source(),
      operation = context.operation(),
      ticket = context.version();
    if (!operation) return false;
    if (route.page === "agents" || route.page === "reviews") return true;
    try {
      if (route.workspace) {
        const value = await readReferences(
          source,
          operation,
          undefined,
          undefined,
          {
            kind: "workspaces",
            exact: route.workspace,
            signal: context.signal?.(),
          },
        );
        if (!context.current(ticket, source)) return false;
        data.workingCopies = unique(
          [
            ...data.workingCopies.filter(
              (item) => item.workspace !== route.workspace,
            ),
            ...value.workingCopies,
          ],
          (item) => item.workspace,
        );
      }
      // Graph detail uses the exact commit; a change choice only selects Files.
      if (route.bookmark || (route.page === "files" && route.changeId)) {
        const kind = route.bookmark ? "bookmarks" : "changes";
        const exact = route.bookmark || route.changeId!;
        const value = await readReferences(
          source,
          operation,
          undefined,
          undefined,
          { kind, exact, signal: context.signal?.() },
        );
        if (!context.current(ticket, source)) return false;
        data.bookmarks = unique(
          [
            ...data.bookmarks.filter((item) => item.name !== route.bookmark),
            ...value.bookmarks,
          ],
          (item) => item.name,
        );
        data.choices = unique(
          [
            ...data.choices.filter(
              (item) =>
                item.value !==
                `${kind === "bookmarks" ? "bookmark" : "change"}:${exact}`,
            ),
            ...value.choices,
          ],
          (item) => item.value,
        );
      }
      if (
        route.page === "branches" ||
        (route.page === "files" &&
          !route.commitId &&
          !route.bookmark &&
          !route.changeId &&
          !route.workspace)
      ) {
        await loadReferences();
        if (!context.current(ticket, source)) return false;
        await loadWorkingCopies();
        return (
          context.current(ticket, source) &&
          data.status.bookmarks !== "failed" &&
          data.workingCopiesStatus !== "failed"
        );
      }
      return true;
    } catch (cause) {
      if (context.current(ticket, source)) context.report(cause);
      return false;
    }
  }
  async function loadReferencePickers() {
    const source = context.source(),
      ticket = context.version(),
      operation = context.operation();
    // Sequential optional windows leave capacity for user-selected content.
    if (
      ["idle", "loading", "refreshing"].includes(data.status.bookmarks) ||
      (data.refsPageSnapshot &&
        snapshotOperation(data.refsPageSnapshot) !== operation)
    )
      await loadReferences(false, true);
    if (!context.current(ticket, source)) return;
    if (
      ["idle", "loading", "refreshing"].includes(data.workingCopiesStatus) ||
      (data.workingCopiesSnapshot &&
        snapshotOperation(data.workingCopiesSnapshot) !== operation)
    )
      await loadWorkingCopies(false, true);
  }
  return {
    loadReferences,
    loadWorkingCopies,
    prepareReferences,
    loadReferencePickers,
  };
}
