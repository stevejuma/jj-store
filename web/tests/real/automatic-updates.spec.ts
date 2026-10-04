import { randomUUID } from "node:crypto";
import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("saved and external task comments appear automatically while drafts survive", async ({
  page,
  context,
  request,
  baseURL,
}) => {
  const home = process.env.JJ_STORE_REAL_REPOSITORY,
    project = process.env.JJ_STORE_REAL_PROJECT,
    task = process.env.JJ_STORE_REAL_TASK;
  test.skip(
    !home || !project || !task,
    "Requires a seeded writable native task",
  );
  test.setTimeout(90000);
  const headers = {
    Origin: new URL(baseURL!).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const route = `/#/projects?home=${home}&project=${project}&task=${task}&tab=tasks`;
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  const observer = await context.newPage();
  observer.on("pageerror", (e) => errors.push(e.message));
  const created: string[] = [];
  const read = async () => {
    const response = await request.get(`/v1/repos/${home}/work?task=${task}`, {
      headers,
    });
    expect(response.ok()).toBe(true);
    return response.json();
  };
  try {
    await page.goto(route);
    await observer.goto(route);
    const detail = page.getByRole("article", { name: "Task details" });
    const other = observer.getByRole("article", { name: "Task details" });
    const editor = (scope: typeof detail) =>
      scope
        .getByRole("group", { name: "Comment", exact: true })
        .getByRole("textbox");
    await expect(detail).toBeVisible();
    await expect(other).toBeVisible();
    await editor(other).fill("Unsent draft survives incoming data");
    const own = `Automatic own save ${randomUUID()}`;
    await editor(detail).fill(own);
    const receipt = page.waitForResponse(
      (r) =>
        r.request().method() === "POST" && r.url().endsWith(`/work/commands`),
    );
    await detail
      .getByRole("button", { name: "Add comment", exact: true })
      .click();
    expect((await receipt).ok()).toBe(true);
    const current = await read();
    created.push(
      current.comments.find((c: { body: string }) => c.body === own).id,
    );
    await expect(
      detail.getByRole("list", { name: "Task comments" }),
    ).toContainText(own);
    await expect(
      other.getByRole("list", { name: "Task comments" }),
    ).toContainText(own, { timeout: 15000 });
    await expect(editor(other)).toHaveText(
      "Unsent draft survives incoming data",
    );
    await expect(
      page.getByRole("status", { name: "Work command status" }),
    ).toHaveCount(0);
    const externalId = randomUUID().replaceAll("-", ""),
      external = `Automatic external save ${randomUUID()}`;
    const response = await request.post(`/v1/repos/${home}/work/commands`, {
      headers,
      data: {
        version: 1,
        request_id: randomUUID().replaceAll("-", ""),
        command: {
          command: "add_comment",
          id: externalId,
          task,
          expected_task_version: current.tasks[0].version,
          reply_to: null,
          body: external,
        },
      },
    });
    expect(response.ok()).toBe(true);
    created.push(externalId);
    await expect(
      detail.getByRole("list", { name: "Task comments" }),
    ).toContainText(external, { timeout: 15000 });
    await expect(
      other.getByRole("list", { name: "Task comments" }),
    ).toContainText(external, { timeout: 15000 });
    await expect(editor(other)).toHaveText(
      "Unsent draft survives incoming data",
    );
    await expect(
      page.getByText("Updates available. Refresh projects to view them."),
    ).toHaveCount(0);
    await expect(
      detail.getByRole("button", { name: "Refresh projects", exact: true }),
    ).toBeEnabled();
    expect(
      (
        await new AxeBuilder({ page })
          .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
          .analyze()
      ).violations,
    ).toEqual([]);
    expect(errors).toEqual([]);
  } finally {
    for (const id of created) {
      const current = await read(),
        comment = current.comments.find((c: { id: string }) => c.id === id);
      if (!comment?.body) continue;
      const response = await request.post(`/v1/repos/${home}/work/commands`, {
        headers,
        data: {
          version: 1,
          request_id: randomUUID().replaceAll("-", ""),
          command: {
            command: "tombstone_comment",
            id,
            expected_version: comment.version,
            expected_task_version: current.tasks[0].version,
          },
        },
      });
      expect(response.ok()).toBe(true);
    }
    await observer.close();
  }
});

test("open task forms keep their original version guards after incoming metadata", async ({
  page,
  request,
  baseURL,
}) => {
  const home = process.env.JJ_STORE_REAL_REPOSITORY,
    project = process.env.JJ_STORE_REAL_PROJECT,
    task = process.env.JJ_STORE_REAL_TASK;
  test.skip(!home || !project || !task, "Requires a seeded native task");
  const headers = {
    Origin: new URL(baseURL!).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const current = await request.get(`/v1/repos/${home}/work?task=${task}`, {
    headers,
  });
  expect(current.ok()).toBe(true);
  const original = await current.json();
  let updated = false;
  await page.route(`**/v1/repos/${home}/snapshot`, async (route) => {
    const response = await route.fetch(),
      body = await response.json();
    if (updated) {
      body.published_seq++;
      body.manifest_revision++;
    }
    await route.fulfill({ response, json: body });
  });
  await page.route(`**/v1/repos/${home}/work?**`, async (route) => {
    const response = await route.fetch(),
      body = await response.json();
    if (updated) {
      body.snapshot.published_seq++;
      for (const row of body.tasks)
        if (row.id === task) {
          row.version++;
          row.title = "Externally edited metadata";
        }
      for (const row of body.projects) if (row.id === project) row.version++;
    }
    await route.fulfill({ response, json: body });
  });
  let sent: any;
  await page.route(`**/v1/repos/${home}/work/commands`, async (route) => {
    sent = route.request().postDataJSON();
    await route.fulfill({
      status: 409,
      json: { version: 1, code: "stale_work" },
    });
  });
  await page.goto(
    `/#/projects?home=${home}&project=${project}&task=${task}&tab=tasks`,
  );
  await page.getByRole("button", { name: "Edit task", exact: true }).click();
  const dialog = page.getByRole("dialog", { name: "Edit task", exact: true });
  const title = dialog.getByRole("textbox", {
    name: "Task title",
    exact: true,
  });
  await title.fill("Unsent task title");
  updated = true;
  const publication = await request.post(`/v1/repos/${home}/work/commands`, {
    headers,
    data: {
      version: 1,
      request_id: randomUUID().replaceAll("-", ""),
      command: {
        command: "add_comment",
        id: randomUUID().replaceAll("-", ""),
        task,
        expected_task_version: original.tasks[0].version,
        reply_to: null,
        body: "Publish an event while the edit form retains its original guards",
      },
    },
  });
  expect(publication.ok()).toBe(true);
  await expect(
    page.locator("article[aria-label='Task details'] h2"),
  ).toContainText("Externally edited metadata", { timeout: 15000 });
  await expect(title).toHaveValue("Unsent task title");
  await dialog.getByRole("button", { name: "Save task", exact: true }).click();
  await expect(dialog.getByRole("alert")).toContainText("stale work");
  expect(sent.command.expected_version).toBe(original.tasks[0].version);
  expect(sent.command.expected_project_version).toBe(
    original.projects[0].version,
  );
  await expect(title).toHaveValue("Unsent task title");
});
