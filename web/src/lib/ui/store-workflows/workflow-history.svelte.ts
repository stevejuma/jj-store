import { untrack } from "svelte";
import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
import type { WorkflowController } from "../../controller/workflows.svelte.js";
import type {
  WorkflowPage,
  WorkflowVersion,
  WorkflowRun,
} from "../../controller/workflow-types.js";
export type WorkflowHistoryEntry = {
  home: string;
  workflow: WorkflowVersion;
  work: WorkflowController;
};
type Group = WorkflowHistoryEntry & {
  rows: WorkflowRun[];
  window?: number;
  page: WorkflowPage | null;
  loading: boolean;
  error: string;
};
const message = (cause: unknown) =>
  cause instanceof Error ? cause.message : "Workflow history unavailable";
const sameSnapshot = (a: WorkflowPage, b: WorkflowPage) =>
  a.repository === b.repository &&
  a.epoch === b.epoch &&
  a.position === b.position &&
  a.root === b.root;
export function createWorkflowHistory(
  entries: () => WorkflowHistoryEntry[],
  owners: () => readonly (readonly [string, WorkflowController])[],
  selection: () => WorkflowSelection,
) {
  const cached = new Map<string, Group>();
  let groups = $state<Group[]>([]),
    generation = 0,
    abort = new AbortController();
  let detail = $state<WorkflowPage | null>(null),
    detailError = $state(""),
    detailLoading = $state(false);
  let detailGeneration = 0,
    detailAbort = new AbortController();
  async function read(group: Group, more = false) {
    const ticket = generation;
    if (group.loading || (more && !group.page?.next)) return;
    if (more) group.window = group.rows.length + 50;
    group.loading = true;
    try {
      const target = more ? 0 : Math.max(group.rows.length, group.window ?? 0);
      let page = await group.work.transport.page(
        {
          workflow: group.workflow.id,
          limit: 50,
          ...(more
            ? { at: group.page!.position, after: group.page!.next! }
            : {}),
        },
        abort.signal,
      );
      if (ticket !== generation) return;
      if (
        page.repository !== group.home ||
        page.runs.some((run) => run.workflow !== group.workflow.id) ||
        (more && !sameSnapshot(group.page!, page))
      )
        throw Error("History snapshot changed. Retry the workflow history.");
      let rows = more ? [...group.rows, ...page.runs] : page.runs;
      while (!more && page.next && rows.length < target) {
        const next = await group.work.transport.page(
          {
            workflow: group.workflow.id,
            at: page.position,
            after: page.next,
            limit: 50,
          },
          abort.signal,
        );
        if (ticket !== generation) return;
        if (
          !sameSnapshot(page, next) ||
          next.runs.some((run) => run.workflow !== group.workflow.id)
        )
          throw Error("History snapshot changed. Retry the workflow history.");
        rows = [...rows, ...next.runs];
        page = next;
      }
      group.rows = [...new Map(rows.map((run) => [run.id, run])).values()];
      group.page = page;
      group.window = group.rows.length;
      group.error = "";
    } catch (cause) {
      if (ticket === generation) group.error = message(cause);
    } finally {
      if (ticket === generation) group.loading = false;
    }
  }
  async function inspect(more = false) {
    const selected = selection(),
      ticket = detailGeneration;
    const owner = owners().find(([home]) => home === selected.home)?.[1];
    const current = () =>
      ticket === detailGeneration &&
      owners().find(([home]) => home === selected.home)?.[1] === owner &&
      ["home", "run", "at", "attempt"].every(
        (key) =>
          selected[key as keyof WorkflowSelection] ===
          selection()[key as keyof WorkflowSelection],
      );
    if (!owner || !selected.run || detailLoading || (more && !detail?.next))
      return;
    detailLoading = true;
    try {
      const target = more ? 0 : (detail?.evidence.length ?? 0);
      let page = await owner.transport.page(
        {
          run: selected.run,
          attempt: selected.attempt,
          at: more
            ? detail!.position
            : selected.at === undefined
              ? undefined
              : BigInt(selected.at),
          after: more ? detail!.next! : undefined,
          limit: 256,
        },
        detailAbort.signal,
      );
      if (!current()) return;
      if (
        page.repository !== selected.home ||
        (selected.at !== undefined && String(page.position) !== selected.at) ||
        page.runs.some((run) => run.id !== selected.run) ||
        page.evidence.some(
          (row) =>
            row.run !== selected.run ||
            (selected.attempt && row.attempt !== selected.attempt),
        ) ||
        (more && !sameSnapshot(detail!, page))
      )
        throw Error("Run snapshot changed. Retry the run details.");
      let evidence = more
        ? [...detail!.evidence, ...page.evidence]
        : page.evidence;
      while (!more && page.next && evidence.length < target) {
        const next = await owner.transport.page(
          {
            run: selected.run,
            attempt: selected.attempt,
            at: page.position,
            after: page.next,
            limit: 256,
          },
          detailAbort.signal,
        );
        if (!current()) return;
        if (
          !sameSnapshot(page, next) ||
          next.runs.some((run) => run.id !== selected.run) ||
          next.evidence.some(
            (row) =>
              row.run !== selected.run ||
              (selected.attempt && row.attempt !== selected.attempt),
          )
        )
          throw Error("Run snapshot changed. Retry the run details.");
        evidence = [...evidence, ...next.evidence];
        page = next;
      }
      detail = {
        ...page,
        evidence: [...new Map(evidence.map((row) => [row.id, row])).values()],
      };
      detailError = "";
    } catch (cause) {
      if (current()) detailError = message(cause);
    } finally {
      if (current()) detailLoading = false;
    }
  }
  $effect(() => {
    const next = entries();
    next.map((entry) => entry.work.state.page?.position);
    untrack(() => {
      abort.abort();
      abort = new AbortController();
      const ticket = ++generation;
      for (const group of groups)
        cached.set(`${group.home}:${group.workflow.id}`, group);
      groups = next.map((entry) => {
        const saved = cached.get(`${entry.home}:${entry.workflow.id}`);
        const previous =
          saved?.work.submit === entry.work.submit ? saved : undefined;
        return {
          ...entry,
          rows: previous?.rows ?? [],
          window: previous?.window,
          page: previous?.page ?? null,
          loading: false,
          error: previous?.error ?? "",
        };
      });
      let index = 0;
      void Promise.all(
        Array.from({ length: Math.min(2, groups.length) }, async () => {
          while (index < groups.length && ticket === generation)
            await read(groups[index++]!);
        }),
      );
    });
    return () => {
      generation++;
      abort.abort();
    };
  });
  $effect(() => {
    const { home, run, at, attempt } = selection();
    const owner = owners().find(([id]) => id === home)?.[1];
    untrack(() => {
      detailGeneration++;
      detailAbort.abort();
      detailAbort = new AbortController();
      detail = null;
      detailError = "";
      detailLoading = false;
      if (owner && run) void inspect();
    });
    return () => {
      detailGeneration++;
      detailAbort.abort();
    };
  });
  return {
    get groups() {
      return groups;
    },
    get detail() {
      return detail;
    },
    get detailError() {
      return detailError;
    },
    get detailLoading() {
      return detailLoading;
    },
    read,
    inspect,
  };
}
