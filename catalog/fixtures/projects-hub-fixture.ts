import { reviewThreads } from "./collaboration-review.js";
import type { WorkBrowseTransport } from "../../web/src/lib/controller/projects-hub-types.js";
export const homeA = "41".repeat(16),
  homeB = "42".repeat(16),
  projectId = "11".repeat(16),
  parentId = "21".repeat(16),
  childId = "22".repeat(16);
const actor = {
  created_by: "01".repeat(16),
  updated_by: "01".repeat(16),
  created_at_millis: 1000,
  updated_at_millis: 2000,
};
export const project = {
  schema_version: 1,
  id: projectId,
  title: "Native release",
  body: "**Release** across two repositories.",
  version: 1,
  repositories: { [homeA]: true, [homeB]: false },
  actor,
};
const parent = {
  schema_version: 1,
  id: parentId,
  project: projectId,
  title: "Storage work",
  body: "Native storage",
  state: "in_progress" as const,
  repository: homeA,
  parent: null,
  version: 1,
  actor,
};
const child = {
  ...parent,
  id: childId,
  title: "Browser work",
  body: "Task discussion",
  state: "blocked" as const,
  repository: homeB,
  parent: parentId,
};
export function fixtureTransport(
  home: string,
  fail = false,
  sharedRepository = false,
): WorkBrowseTransport {
  const snapshot = {
    repo_id: home,
    epoch: "33".repeat(16),
    published_seq: 1,
    manifest_revision: 1,
    work_root: null,
    work_state_version: 1,
  };
  return {
    async directory() {
      const transport = fixtureTransport(home, fail, sharedRepository);
      const tasks = await transport.browse({ kind: "tasks" });
      const reviews = await transport.reviews!();
      return {
        code: {
          version: 1,
          repo_id: home,
          epoch: snapshot.epoch,
          published_seq: 1,
          manifest_revision: 1,
          code_state_version: 1,
          heads: ["61".repeat(64)],
          operation_id: "61".repeat(64),
        },
        work: tasks.snapshot,
        review: reviews.snapshot,
        description:
          home === homeA
            ? "Native parsing and storage services"
            : "Community browser interface",
        contributor: {
          name: "Maya Chen",
          email: "maya@example.test",
          timestamp: { millis: Date.UTC(2026, 8, 29), offset_minutes: 0 },
        },
        updatedAtMillis: Date.UTC(2026, 8, 30),
        commits: home === homeA ? 24 : 12,
        commitsPartial: false,
        threads: reviews.threads,
        reviewsPartial: false,
        tasks: tasks.tasks,
        tasksPartial: false,
      };
    },
    async browse(query) {
      if (fail && home === homeB) throw new Error("Home unavailable");
      const tasks = [parent, child]
        .filter(
          (t) =>
            !query.q || t.title.toLowerCase().includes(query.q.toLowerCase()),
        )
        .filter((t) => !query.parent || t.parent === query.parent)
        .map((t) => ({
          ...t,
          project_title: project.title,
          parent_title: t.parent ? parent.title : null,
          comment_count: 1,
          subtask_count: t.id === parentId ? 1 : 0,
        }));
      const projects =
        home === homeA
          ? [
              {
                ...project,
                excerpt: project.body,
                task_counts: { active: 2, done: 0, total: 2 },
              },
            ]
          : [];
      if (sharedRepository && home === homeA && projects[0])
        projects.push({
          ...projects[0],
          id: "12".repeat(16),
          title: "Shared code project",
        });
      return {
        version: 1,
        snapshot,
        projects: query.kind === "tasks" ? [] : projects,
        tasks: query.kind === "tasks" ? tasks : [],
        total_matches: query.kind === "tasks" ? tasks.length : projects.length,
        next_cursor: null,
      };
    },
    async reviews() {
      return {
        version: 1,
        snapshot: {
          repo_id: home,
          epoch: snapshot.epoch,
          published_seq: 1,
          manifest_revision: 1,
          review_root: "55".repeat(32),
          review_state_version: 1,
        },
        threads:
          home === homeA
            ? reviewThreads.slice(0, 1).map((thread) => ({
                ...thread,
                anchor: { ...thread.anchor, repository_id: home },
              }))
            : [],
        next_cursor: null,
      };
    },
    async page(query) {
      return {
        version: 1,
        snapshot,
        projects: [project],
        tasks: query.task
          ? [query.task === parentId ? parent : child]
          : [parent, child],
        comments: query.task
          ? [
              {
                schema_version: 1,
                id: "31".repeat(16),
                task: query.task,
                reply_to: null,
                body: "First discussion",
                version: 1,
                actor,
              },
            ]
          : [],
        next_cursor: null,
      };
    },
    async apply(request) {
      return { status: "outcome_unknown", request_id: request.request_id };
    },
    async status(request_id) {
      return { status: "not_known", request_id };
    },
  };
}
