import type { StoreController } from "../../controller/contracts.js";
import type { SessionInboxItem } from "../community-inbox/model.js";
import { principalMember } from "../../collaboration/presentation.js";

/** One metadata authority, with display aliases under independently authorized Codebase members. */
export function threadInboxRows(
  controller: StoreController,
): SessionInboxItem[] {
  const rows: SessionInboxItem[] = [];
  const repositories = controller.state.repositories;
  for (const [home, entry] of Object.entries(
    controller.threadNavigation?.entries ?? {},
  )) {
    for (const thread of entry.threads) {
      if (
        thread.value.purpose ||
        thread.value.agent_message ||
        thread.value.comment_source
      )
        continue;
      const identity = `${home}:${thread.value.thread_id}`;
      const codebase = thread.value.codebase ?? [home];
      const visible = codebase.filter((id) =>
        repositories.some((r) => r.id === id && r.permission),
      );
      for (const repository of visible.length
        ? visible
        : codebase.length
          ? []
          : [null]) {
        const repo = repositories.find((r) => r.id === repository);
        rows.push({
          id: `${identity}:${repository ?? "unbound"}`,
          targetId: identity,
          author: principalMember(thread.value.actor),
          label: thread.value.title,
          context: thread.value.title,
          excerpt: thread.directory?.excerpt,
          status: thread.value.status,
          lastActivityAt:
            Number(
              thread.directory?.last_activity_at_millis ??
                thread.value.accepted_at_millis,
            ) / 1000,
          ...(repo
            ? {
                repository: {
                  id: repo.id,
                  name: repo.name,
                  href: `#/repos/${repo.id}/files`,
                  label: `Open ${repo.name}`,
                },
              }
            : {}),
        });
      }
    }
  }
  return rows.sort((a, b) => (b.lastActivityAt ?? 0) - (a.lastActivityAt ?? 0));
}
