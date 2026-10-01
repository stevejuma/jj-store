import { sampleWork, workSeedId } from "./dev-demo-work-samples.mjs";

export async function seedWork({
  origin,
  repositoryId,
  repositoryLinks = [repositoryId, repositoryId, repositoryId],
  fetcher = fetch,
}) {
  if (!/^[0-9a-f]{32}$/u.test(repositoryId))
    throw new Error("invalid demo repository ID");
  if (
    repositoryLinks.length !== 3 ||
    repositoryLinks.some((id) => !/^[0-9a-f]{32}$/u.test(id))
  )
    throw new Error("invalid demo repository links");
  const base = `${origin}v1/repos/${repositoryId}/`;
  const headers = {
    Origin: new URL(origin).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const api = async (path, options = {}) => {
    const response = await fetcher(`${base}${path}`, {
      credentials: "omit",
      redirect: "error",
      cache: "no-store",
      signal: AbortSignal.timeout(30000),
      ...options,
      headers: { ...headers, ...options.headers },
    });
    if (!response.ok)
      throw new Error(
        `development work seed failed: ${response.status} ${await response.text()}`,
      );
    return response.json();
  };
  const page = async (query = {}) => {
    const result = { projects: [], tasks: [], comments: [] };
    const cursors = new Set();
    let cursor;
    do {
      const parameters = new URLSearchParams({
        ...query,
        limit: "256",
        ...(cursor ? { cursor } : {}),
      });
      const current = await api(`work?${parameters}`);
      for (const key of Object.keys(result)) result[key].push(...current[key]);
      cursor = current.next_cursor;
      if (cursor && cursors.has(cursor))
        throw new Error("development work seed received a repeated cursor");
      cursors.add(cursor);
    } while (cursor);
    return result;
  };
  let publications = 0;
  const apply = async (command) => {
    const requestId = workSeedId(
      repositoryId,
      "request",
      JSON.stringify(command),
    );
    const outcome = await api("work/commands", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ version: 1, request_id: requestId, command }),
    });
    if (outcome.status !== "committed")
      throw new Error(
        `development work seed outcome was ${outcome.status}; request ${requestId}; rerun to inspect committed records and resume`,
      );
    publications += 1;
    return outcome.receipt.result;
  };
  const samples = sampleWork(repositoryId, repositoryLinks);
  const projects = new Map((await page()).projects.map((p) => [p.id, p]));
  for (const sample of samples) {
    let project = projects.get(sample.id);
    if (!project)
      project = (
        await apply({
          command: "create_project",
          id: sample.id,
          title: sample.title,
          body: sample.body,
        })
      ).project;
    const migrate =
      sample.repository !== repositoryId &&
      project.title === sample.title &&
      project.body === sample.body &&
      project.repositories[repositoryId] === true &&
      Object.entries(project.repositories).every(
        ([id, active]) =>
          active && [repositoryId, sample.repository].includes(id),
      );
    if (
      (!projects.has(sample.id) ||
        sample.repository === repositoryId ||
        migrate) &&
      !Object.hasOwn(project.repositories, sample.repository)
    )
      project = (
        await apply({
          command: "set_repository",
          project: project.id,
          expected_version: project.version,
          repository: sample.repository,
          active: true,
        })
      ).project;
    const tasks = new Map(
      (await page({ project: project.id })).tasks.map((t) => [t.id, t]),
    );
    for (const item of sample.tasks) {
      let task = tasks.get(item.id);
      const metadata = {
        title: item.title,
        body: item.body,
        repository: item.repository,
        parent: item.parent,
      };
      if (!task)
        task = (
          await apply({
            command: "create_task",
            id: item.id,
            project: project.id,
            expected_project_version: project.version,
            ...metadata,
          })
        ).task;
      // Resume an interrupted initial state change, never overwrite an edited task.
      if (
        task.version === 1 &&
        item.state !== "open" &&
        Object.entries(metadata).every(([key, value]) => task[key] === value)
      )
        task = (
          await apply({
            command: "update_task",
            id: task.id,
            expected_version: task.version,
            expected_project_version: project.version,
            ...metadata,
            state: item.state,
          })
        ).task;
      if (item.comments.length) {
        const current = await page({ project: project.id, task: task.id });
        task = current.tasks[0];
        const comments = new Set(current.comments.map((c) => c.id));
        for (const comment of item.comments) {
          if (comments.has(comment.id)) continue;
          task = (
            await apply({
              command: "add_comment",
              id: comment.id,
              task: task.id,
              expected_task_version: task.version,
              reply_to: comment.reply_to,
              body: comment.body,
            })
          ).task;
        }
      }
      if (
        migrate &&
        task.repository === repositoryId &&
        task.version ===
          1 + (item.state === "open" ? 0 : 1) + item.comments.length &&
        task.state === item.state &&
        task.title === item.title &&
        task.body === item.body &&
        task.parent === item.parent &&
        item.repository
      ) {
        task = (
          await apply({
            command: "update_task",
            id: task.id,
            expected_version: task.version,
            expected_project_version: project.version,
            ...metadata,
            state: task.state,
          })
        ).task;
      }
      tasks.set(task.id, task);
    }
    if (
      migrate &&
      ![...tasks.values()].some((task) => task.repository === repositoryId)
    ) {
      project = (
        await apply({
          command: "set_repository",
          project: project.id,
          expected_version: project.version,
          repository: repositoryId,
          active: false,
        })
      ).project;
    }
  }
  return {
    projects: samples.length,
    tasks: samples.reduce((count, p) => count + p.tasks.length, 0),
    comments: samples.reduce(
      (count, p) => count + p.tasks.reduce((n, t) => n + t.comments.length, 0),
      0,
    ),
    publications,
  };
}
