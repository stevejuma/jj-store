import assert from "node:assert/strict";
import test from "node:test";
import { seedWork } from "./dev-demo-work.mjs";

const options = {
  origin: "http://127.0.0.1:7460/",
  repositoryId: "0123456789abcdef0123456789abcdef",
};

function service() {
  const projects = new Map(),
    tasks = new Map(),
    comments = new Map();
  const requests = [];
  let unknown;
  const fetcher = async (input, init) => {
    const url = new URL(input);
    assert.equal(init.headers.Origin, "http://127.0.0.1:7460");
    assert.equal(init.headers["X-JJ-Store-Protocol"], "1");
    if (init.method !== "POST") {
      const project = url.searchParams.get("project"),
        task = url.searchParams.get("task");
      return Response.json({
        projects: [...projects.values()].filter(
          (p) => !project || p.id === project,
        ),
        tasks: [...tasks.values()].filter((t) =>
          task ? t.id === task : t.project === project,
        ),
        comments: [...comments.values()].filter((c) => c.task === task),
        next_cursor: null,
      });
    }
    const request = JSON.parse(init.body),
      command = request.command;
    requests.push(request);
    assert.match(request.request_id, /^[0-9a-f]{32}$/u);
    const result = {};
    if (command.command === "create_project") {
      assert.equal(projects.has(command.id), false);
      result.project = { ...command, version: 1, repositories: {} };
      projects.set(command.id, result.project);
    } else if (command.command === "set_repository") {
      const project = projects.get(command.project);
      assert.equal(command.expected_version, project.version);
      result.project = {
        ...project,
        version: project.version + 1,
        repositories: {
          ...project.repositories,
          [command.repository]: command.active,
        },
      };
      projects.set(project.id, result.project);
    } else if (command.command === "create_task") {
      assert.equal(
        command.expected_project_version,
        projects.get(command.project).version,
      );
      if (command.repository)
        assert.equal(
          projects.get(command.project).repositories[command.repository],
          true,
        );
      if (command.parent)
        assert.equal(tasks.get(command.parent).project, command.project);
      assert.equal(tasks.has(command.id), false);
      result.task = { ...command, state: "open", version: 1 };
      tasks.set(command.id, result.task);
    } else if (command.command === "update_task") {
      assert.equal(command.expected_version, tasks.get(command.id).version);
      result.task = {
        ...tasks.get(command.id),
        ...command,
        version: command.expected_version + 1,
      };
      tasks.set(command.id, result.task);
    } else if (command.command === "add_comment") {
      const task = tasks.get(command.task);
      assert.equal(command.expected_task_version, task.version);
      if (command.reply_to)
        assert.equal(comments.get(command.reply_to).task, task.id);
      assert.equal(comments.has(command.id), false);
      result.comment = { ...command, version: 1 };
      result.task = { ...task, version: task.version + 1 };
      comments.set(command.id, result.comment);
      tasks.set(task.id, result.task);
    } else assert.fail(`unexpected seed command ${command.command}`);
    if (unknown === command.command) {
      unknown = undefined;
      return Response.json({
        status: "outcome_unknown",
        request_id: request.request_id,
      });
    }
    return Response.json({ status: "committed", receipt: { result } });
  };
  return {
    projects,
    tasks,
    comments,
    requests,
    fetcher,
    loseNextReceipt: (command = "create_project") => {
      unknown = command;
    },
  };
}

test("work seed adds linked projects, all task states, subtasks and threaded discussion once", async () => {
  const state = service();
  await seedWork({ ...options, fetcher: state.fetcher });
  assert.equal(state.projects.size, 3);
  assert.equal(state.tasks.size, 9);
  assert.equal(state.comments.size, 6);
  assert.deepEqual(
    new Set([...state.tasks.values()].map((t) => t.state)),
    new Set(["open", "in_progress", "blocked", "done"]),
  );
  assert.equal([...state.tasks.values()].filter((t) => t.parent).length, 3);
  assert.equal(
    [...state.comments.values()].filter((c) => c.reply_to).length,
    3,
  );
  const count = state.requests.length;
  await seedWork({ ...options, fetcher: state.fetcher });
  assert.equal(
    state.requests.length,
    count,
    "reseed must not publish duplicate work",
  );
});

test("work seed preserves edited records, retired links and comment tombstones", async () => {
  const state = service();
  await seedWork({ ...options, fetcher: state.fetcher });
  const project = [...state.projects.values()][0],
    task = [...state.tasks.values()][0],
    comment = [...state.comments.values()][0];
  project.title = "My project title";
  project.repositories[options.repositoryId] = false;
  project.version += 1;
  task.title = "My task title";
  task.state = "open";
  task.version += 1;
  comment.body = null;
  comment.version += 1;
  const before = JSON.stringify([
    ...state.projects,
    ...state.tasks,
    ...state.comments,
  ]);
  const count = state.requests.length;
  await seedWork({ ...options, fetcher: state.fetcher });
  assert.equal(state.requests.length, count);
  assert.equal(
    JSON.stringify([...state.projects, ...state.tasks, ...state.comments]),
    before,
  );
});

test("work seed stops on an unknown outcome and resumes from committed records", async () => {
  const state = service();
  state.loseNextReceipt();
  await assert.rejects(
    seedWork({ ...options, fetcher: state.fetcher }),
    /outcome_unknown/u,
  );
  assert.equal(state.requests.length, 1);
  await seedWork({ ...options, fetcher: state.fetcher });
  assert.equal(state.projects.size, 3);
  assert.equal(state.tasks.size, 9);
  assert.equal(state.comments.size, 6);
  assert.equal(
    state.requests.filter((r) => r.command.command === "create_project").length,
    3,
  );
});

test("work seed resumes an interrupted task creation before its initial state update", async () => {
  const state = service();
  state.loseNextReceipt("create_task");
  await assert.rejects(
    seedWork({ ...options, fetcher: state.fetcher }),
    /outcome_unknown/u,
  );
  const task = [...state.tasks.values()][0];
  assert.equal(task.state, "open");
  assert.equal(task.version, 1);
  await seedWork({ ...options, fetcher: state.fetcher });
  assert.equal(state.tasks.get(task.id).state, "in_progress");
  assert.equal(
    state.requests.filter((r) => r.command.command === "create_task").length,
    9,
  );
});
