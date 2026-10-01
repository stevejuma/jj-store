import { expect, test } from "@playwright/test";
import { execFileSync } from "node:child_process";

test("development work samples survive cold startup and additive reseeding", async ({
  page,
  request,
}) => {
  const home = process.env.JJ_STORE_REAL_REPOSITORY!;
  const base = `/v1/repos/${home}/work`;
  const headers = {
    Origin: new URL(process.env.JJ_STORE_REAL_BASE_URL!).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const read = async (query = "") => {
    const response = await request.get(`${base}?limit=256${query}`, {
      headers,
    });
    expect(response.ok()).toBe(true);
    return response.json();
  };
  const projects = await read();
  const titles = [
    "Native repository browser",
    "Recovery and reliability",
    "Developer experience",
  ];
  const samples = projects.projects.filter((p: { title: string }) =>
    titles.includes(p.title),
  );
  expect(samples).toHaveLength(3);
  const states = new Set(),
    taskIds = new Set();
  let subtasks = 0,
    comments = 0,
    replies = 0;
  let browsingTaskId = "";
  for (const project of samples) {
    expect(project.repositories[home]).toBe(true);
    const tasks = await read(`&project=${project.id}`);
    expect(tasks.tasks).toHaveLength(3);
    for (const task of tasks.tasks) {
      states.add(task.state);
      taskIds.add(task.id);
      if (task.title === "Ship repository browsing") browsingTaskId = task.id;
      if (task.parent) subtasks += 1;
      const discussion = await read(`&project=${project.id}&task=${task.id}`);
      comments += discussion.comments.length;
      replies += discussion.comments.filter(
        (c: { reply_to: string | null }) => c.reply_to,
      ).length;
    }
  }
  expect(taskIds.size).toBe(9);
  expect(states).toEqual(new Set(["open", "in_progress", "blocked", "done"]));
  expect(subtasks).toBe(3);
  expect(comments).toBe(6);
  expect(replies).toBe(3);
  const output = execFileSync("pnpm", ["dev:seed:work"], {
    cwd: process.env.JJ_STORE_REAL_ROOT!,
    encoding: "utf8",
    env: {
      ...process.env,
      JJ_STORE_DEV_ROOT: process.env.JJ_STORE_REAL_RUNTIME!,
      JJ_STORE_DEV_PORT: new URL(process.env.JJ_STORE_REAL_BASE_URL!).port,
    },
  });
  expect(output).toContain("0 new publications");
  expect((await read()).snapshot).toEqual(projects.snapshot);

  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto("/#/projects");
  const view = page.getByRole("region", { name: "Projects and tasks" });
  await view
    .getByRole("button", {
      name: "Open project Native repository browser",
      exact: true,
    })
    .click();
  await view
    .getByRole("button", {
      name: "Open task Ship repository browsing",
      exact: true,
    })
    .click();
  await expect(
    view.getByRole("heading", {
      name: `Ship repository browsing #${browsingTaskId.slice(0, 8)}`,
      exact: true,
    }),
  ).toBeVisible();
  expect(
    new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get("task"),
  ).toBe(browsingTaskId);
  await view.getByRole("button", { name: "Activity", exact: true }).click();
  await expect(
    view.getByText("The list and card layouts are ready for a visual pass.", {
      exact: false,
    }),
  ).toBeVisible();
  await expect(
    view.getByText(
      "Please include keyboard navigation through the repository picker",
      { exact: false },
    ),
  ).toBeVisible();
  expect(errors).toEqual([]);
});
