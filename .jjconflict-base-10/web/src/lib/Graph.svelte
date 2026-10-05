<script lang="ts">
  import type { Graph } from "./api/client";
  import { rows, layout } from "./graph";
  let {
    graph,
    selected,
    select,
  }: { graph: Graph; selected: string; select: (id: string) => void } =
    $props();
  const nodes = $derived(rows(graph));
  const drawing = $derived(layout(nodes));
</script>

<div class="graph-scroll">
  <div class="graph-grid">
    <svg
      class="graph-lines"
      width={drawing.width}
      height={nodes.length * 96}
      aria-hidden="true"
    >
      {#each drawing.edges as edge}
        <path
          d={`M${edge.from.x},${edge.from.y} C${edge.from.x},${edge.from.y + 36} ${edge.to.x},${edge.to.y - 36} ${edge.to.x},${edge.to.y}`}
          class:dashed={edge.dashed}
        />
      {/each}
      {#each nodes as node}
        {@const position = drawing.positions.get(node.id)!}
        <circle
          cx={position.x}
          cy={position.y}
          r="5"
          class:chosen={node.id === selected}
        />
      {/each}
    </svg>
    <ol class="graph-rows">
      {#each nodes as node (node.id)}
        <li>
          <button
            class="graph-row"
            class:chosen={selected === node.id}
            onclick={() => select(node.id)}
            aria-pressed={selected === node.id}
          >
            <span class="row-title"
              >{node.title.split("\n")[0] || "No description"}</span
            >
            <span class="row-meta"
              ><code title={node.id}>{node.id.slice(0, 12)}</code> · {node.subtitle}</span
            >
            <span class="badges"
              >{#each node.badges as badge}<span class="badge">{badge}</span
                >{/each}</span
            >
          </button>
        </li>
      {/each}
    </ol>
  </div>
</div>
