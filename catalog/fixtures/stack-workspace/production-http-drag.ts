import { base, topic, stackOperation } from "./production-http-data.js";
const json = (value: unknown) =>
  new Response(JSON.stringify(value), {
    headers: { "content-type": "application/json" },
  });
export const productionDragLane = {
  applied: ["topic"],
  unapply: ["topic"],
  heads: [
    {
      names: ["topic"],
      commit: topic,
      base_commit: base,
      shared: false,
    },
  ],
};
/** A deterministic native command boundary; unconfirmed execution is forbidden. */
export function productionDragResponse(path: string, body: any) {
  if (path.endsWith("/stacks")) {
    if (body.command.kind !== "refresh")
      throw new Error(
        "Drag/drop must not execute a native command before confirmation.",
      );
    return json({
      version: 1,
      operation: stackOperation,
      revision: 0,
      active: "work",
      warning: null,
      workspaces: [
        {
          id: "work",
          name: "Work",
          target: "main",
          target_commit: base,
          applied: ["topic"],
          merge_commit: "4".repeat(40),
          wip_commit: "5".repeat(40),
          conflicted: false,
        },
      ],
    });
  }
  if (path.endsWith("/stacks/inspect"))
    return json({
      commits: body.commits.map((commit: string) => ({
        commit,
        mutable: true,
        reason: null,
      })),
    });
  if (path.endsWith("/stacks/hunks"))
    return json({
      source: body.source,
      path: body.path,
      reason: null,
      hunks: [
        {
          id: "native:" + "a".repeat(64),
          label: "@@ -1 +1 @@",
          patch: "@@ -1 +1 @@\n-old\n+new\n",
        },
      ],
    });
  if (path.endsWith("/stacks/preview"))
    return json({
      title: "Review native transfer",
      summary: `Move ${body.command.selections[0].path} from ${body.command.selections[0].source}`,
      paths: [body.command.selections[0].path],
    });
  return null;
}
