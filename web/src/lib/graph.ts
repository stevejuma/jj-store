import type { Graph } from "./api/client";
export interface Row {
  id: string;
  title: string;
  subtitle: string;
  badges: string[];
  edges: { target: string; indirect: boolean }[];
}
export function rows(graph: Graph): Row[] {
  switch (graph.data.graph) {
    case "log":
      return graph.data.nodes.map(({ commit: c, edges }) => ({
        id: c.id,
        title: c.description,
        subtitle: c.author.name,
        badges: [
          ...c.bookmarks.map(
            (b) => `${b.name}${b.conflicted ? " (conflicted)" : ""}`,
          ),
          ...c.workspaces.map((w) => `workspace ${w}`),
          ...(c.conflicted ? ["conflicts"] : []),
          ...(c.divergent ? ["divergent"] : []),
          ...(!c.visible ? ["hidden"] : []),
        ],
        edges: edges.map((e) => ({
          target: e.target,
          indirect: e.kind !== "direct",
        })),
      }));
    case "operations":
      return graph.data.nodes.map((o) => ({
        id: o.id,
        title: o.description,
        subtitle: o.user,
        badges: [
          ...(o.parents.length > 1 ? ["merged operation"] : []),
          ...(o.is_snapshot ? ["snapshot"] : []),
        ],
        edges: o.parents.map((target) => ({ target, indirect: false })),
      }));
    case "evolution":
      return graph.data.nodes.map((e) => ({
        id: e.commit.id,
        title: e.commit.description,
        subtitle: e.commit.author.name,
        badges: [
          ...(!e.commit.visible ? ["hidden"] : []),
          ...(!e.predecessors_known ? ["predecessors unknown"] : []),
        ],
        edges: e.predecessors.map((target) => ({ target, indirect: false })),
      }));
  }
}
export function layout(rows: Row[]) {
  const lanes: (string | undefined)[] = [];
  const positions = new Map<string, { x: number; y: number }>();
  for (const [index, row] of rows.entries()) {
    let lane = lanes.indexOf(row.id);
    if (lane < 0) {
      lane = lanes.indexOf(undefined);
      if (lane < 0) lane = lanes.length;
    }
    positions.set(row.id, { x: 20 + lane * 18, y: index * 96 + 32 });
    lanes[lane] = undefined;
    for (const edge of row.edges)
      if (!lanes.includes(edge.target)) {
        let next = lanes.indexOf(undefined);
        if (next < 0) next = lanes.length;
        lanes[next] = edge.target;
      }
  }
  const width = Math.max(72, ...[...positions.values()].map((p) => p.x + 24));
  return {
    width,
    positions,
    edges: rows.flatMap((r) =>
      r.edges.map((e) => ({
        from: positions.get(r.id)!,
        to: positions.get(e.target) ?? {
          x: positions.get(r.id)!.x,
          y: rows.length * 96,
        },
        dashed: e.indirect || !positions.has(e.target),
        target: e.target,
      })),
    ),
  };
}

/** Bound DOM/layout work before allocating graph rows, including resumed pages. */
export function displayable(graph: Graph): boolean {
  if (graph.data.nodes.length > 1000) return false;
  let edges = 0;
  switch (graph.data.graph) {
    case "log":
      edges = graph.data.nodes.reduce((n, node) => n + node.edges.length, 0);
      break;
    case "operations":
      edges = graph.data.nodes.reduce((n, node) => n + node.parents.length, 0);
      break;
    case "evolution":
      edges = graph.data.nodes.reduce(
        (n, node) => n + node.predecessors.length,
        0,
      );
      break;
  }
  return edges <= 4096;
}
