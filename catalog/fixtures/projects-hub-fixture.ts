import { agentSessions } from "./collaboration.js";
import { reviewThreads } from "./collaboration-review.js";
import { activityCommit } from "./native-activity.js";
import type { WorkActivityPage } from "../../web/src/lib/controller/work-types.js";
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
    async agents() {
      return {
        version: 1,
        snapshot: {
          repo_id: home,
          epoch: snapshot.epoch,
          published_seq: 1,
          manifest_revision: 1,
          agent_root: "56".repeat(32),
          agent_state_version: 1,
        },
        sessions: [agentSessions[home === homeA ? 0 : 1]!],
        next_cursor: null,
      };
    },
    async activity() {
      const commit = activityCommit(home);
      return {
        snapshot: {
          version: 1,
          repo_id: home,
          epoch: snapshot.epoch,
          published_seq: 1,
          manifest_revision: 1,
          code_state_version: 1,
          heads: [commit.operation],
          operation_id: commit.operation,
        },
        atTimeMillis: Date.now(),
        commits: home === homeA ? [commit] : [],
        cursor: null,
      };
    },
    async directory() {
      const transport = fixtureTransport(home, fail, sharedRepository);
      const tasks = await transport.taskActivity!({ limit: 256 });
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
        taskGroups: tasks.groups,
        tasksPartial: false,
      };
    },
    async taskActivity(query) {
      const tasks = await fixtureTransport(home, fail, sharedRepository).browse(
        { kind: "tasks" },
      );
      if (!("epoch" in tasks.snapshot))
        throw new Error("Expected native fixture snapshot");
      const groups = new Map<string, WorkActivityPage["groups"][number]>();
      for (const task of tasks.tasks) {
        const key = `${task.repository ?? ""}:${task.project}`;
        const group = groups.get(key) ?? {
          repository: task.repository,
          project: task.project,
          tasks: 0,
          updated_at_millis: task.actor.updated_at_millis,
        };
        group.tasks++;
        group.updated_at_millis = Math.max(
          Number(group.updated_at_millis),
          Number(task.actor.updated_at_millis),
        );
        groups.set(key, group);
      }
      const all = [...groups.values()].sort(
        (a, b) =>
          (a.repository ?? "").localeCompare(b.repository ?? "") ||
          a.project.localeCompare(b.project),
      );
      const offset = Number(query.cursor ?? 0),
        end = offset + (query.limit ?? 50);
      return {
        version: 1,
        snapshot: tasks.snapshot,
        groups: all.slice(offset, end),
        total_groups: all.length,
        next_cursor: end < all.length ? String(end) : null,
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
