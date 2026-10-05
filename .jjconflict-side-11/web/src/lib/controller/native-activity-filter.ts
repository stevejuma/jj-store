import type { NativeActivityCommit } from "./native-activity-types.js";

export type NativeActivityFilter = "all" | "agent" | "unassisted";
export const nativeActivityFilters = [
  {
    id: "all",
    label: "All commits",
    description: "Every commit in the selected repositories",
  },
  {
    id: "agent",
    label: "With agent context",
    description: "Commits with recorded agent associations",
  },
  {
    id: "unassisted",
    label: "Without agent context",
    description: "Commits with no recorded agent associations",
  },
] as const;

/** Classification reads association metadata, never session bodies. */
export async function filterNativeActivity(
  rows: readonly NativeActivityCommit[],
  filter: NativeActivityFilter,
  current: () => boolean = () => true,
): Promise<{ rows: NativeActivityCommit[]; unavailable: number }> {
  if (filter === "all") return { rows: [...rows], unavailable: 0 };
  const selected: (NativeActivityCommit | undefined)[] = new Array(rows.length);
  let index = 0,
    unavailable = 0;
  await Promise.all(
    Array.from({ length: Math.min(2, rows.length) }, async () => {
      while (index < rows.length && current()) {
        const position = index++,
          row = rows[position]!;
        try {
          const agents = row.agents.discover
            ? await row.agents.discover()
            : row.agents;
          if (!current()) return;
          if (agents.error) {
            unavailable++;
            continue;
          }
          const associated = Boolean(
            agents.sessions.length || agents.relatedSessionIds?.length,
          );
          if ((filter === "agent") === associated)
            selected[position] = { ...row, agents };
        } catch {
          if (current()) unavailable++;
        }
      }
    }),
  );
  return {
    rows: selected.filter((row): row is NativeActivityCommit => Boolean(row)),
    unavailable,
  };
}
