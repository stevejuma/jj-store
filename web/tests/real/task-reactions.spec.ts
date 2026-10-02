import { randomUUID } from "node:crypto";
import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("task reaction picker persists exact comments and refreshes a second client", async ({
  page,
  context,
  request,
  baseURL,
}) => {
  test.setTimeout(120000);
  page.setDefaultTimeout(15000);
  const home = process.env.JJ_STORE_REAL_REPOSITORY!;
  const project = randomUUID().replaceAll("-", ""),
    task = randomUUID().replaceAll("-", ""),
    comment = randomUUID().replaceAll("-", "");
  const headers = {
    Origin: new URL(baseURL!).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const apply = async (command: unknown) => {
    const r = await request.post(`/v1/repos/${home}/work/commands`, {
      headers,
      data: {
        version: 1,
        request_id: randomUUID().replaceAll("-", ""),
        command,
      },
    });
    expect(r.ok(), await r.text()).toBe(true);
  };
  await apply({
    command: "create_project",
    id: project,
    title: "Reaction acceptance",
    body: "Disposable browser qualification",
  });
  await apply({
    command: "create_task",
    id: task,
    project,
    expected_project_version: 1,
    title: "Task reaction acceptance",
    body: "",
    repository: null,
    parent: null,
  });
  await apply({
    command: "add_comment",
    id: comment,
    task,
    expected_task_version: 1,
    reply_to: null,
    body: "Persisted task reaction",
  });
  const route = `/#/projects?home=${home}&project=${project}&task=${task}&tab=tasks`;
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  const observer = await context.newPage();
  observer.on("pageerror", (e) => errors.push(e.message));
  await page.goto(route);
  await observer.goto(route);
  const row = page.getByRole("listitem", {
    name: `Comment ${comment.slice(0, 8)}`,
    exact: true,
  });
  const other = observer.getByRole("listitem", {
    name: `Comment ${comment.slice(0, 8)}`,
    exact: true,
  });
  const reactions = row.getByRole("group", {
    name: "Message reactions",
    exact: true,
  });
  await expect(row).toContainText("Persisted task reaction");
  const revealActions = async () => {
    const message = row.locator(".community-message");
    await message.hover();
    await expect(message).toHaveAttribute("data-actions-visible", "");
  };
  for (const name of ["Edit task", "Add subtask"]) {
    const button = page.getByRole("button", { name, exact: true });
    const rest = await button.evaluate(
      (e) => getComputedStyle(e).backgroundColor,
    );
    expect(rest).not.toBe("rgba(0, 0, 0, 0)");
    await button.hover();
    await expect
      .poll(() => button.evaluate((e) => getComputedStyle(e).backgroundColor))
      .not.toBe(rest);
  }
  await revealActions();
  await row.getByRole("button", { name: "More message actions" }).click();
  const copy = page.getByRole("menuitem", { name: "Copy", exact: true });
  const edit = page.getByRole("menuitem", {
    name: "Edit comment",
    exact: true,
  });
  const iconX = (scope: typeof copy) =>
    scope
      .locator("svg")
      .first()
      .evaluate((e) => e.getBoundingClientRect().left);
  expect(
    Math.abs((await iconX(copy)) - (await iconX(edit))),
  ).toBeLessThanOrEqual(1);
  await copy.press("Escape");
  await revealActions();
  await row.getByRole("button", { name: "Add reaction", exact: true }).click();
  const picker = page.locator(".community-emoji-picker__content");
  await expect(picker).toBeVisible();
  const search = picker.locator('input[type="search"]');
  await search.fill("thumbs up");
  await picker.getByRole("button", { name: "👍", exact: true }).first().click();
  const bar = row.getByRole("button", { name: /1 reaction with like/ });
  await expect(bar).toBeVisible();
  await expect(bar).toHaveAttribute("aria-pressed", "true");
  await expect(
    other.getByRole("button", { name: /1 reaction with like/ }),
  ).toBeVisible();
  await page.reload();
  await expect(bar).toBeVisible();
  const target = {
    type: "work_comment",
    task_id: task,
    comment_id: comment,
    version: 1,
  };
  const query = () =>
    request.post(`/v1/repos/${home}/reactions/query`, {
      headers,
      data: { target, limit: 50 },
    });
  const loaded = await query();
  expect(loaded.ok()).toBe(true);
  const saved = await loaded.json();
  expect(saved.items.target).toEqual(target);
  expect(saved.items.reactions).toMatchObject([
    { count: 1, viewer_reacted: true },
  ]);
  expect(saved.snapshot.work_root).toBeTruthy();
  await bar.click();
  await expect(bar).toHaveCount(0);
  await expect(
    other.getByRole("button", { name: /1 reaction with like/ }),
  ).toHaveCount(0);
  // Lost acknowledgement retains the actual committed request for status recovery.
  let retained: { request_id: string; command: unknown } | undefined;
  const loseAcknowledgement = async () => {
    let confirm!: () => void;
    let fail!: (error: unknown) => void;
    const committed = new Promise<void>((resolve, reject) => {
      confirm = resolve;
      fail = reject;
    });
    await page.route(
      "**/reactions/commands",
      async (r) => {
        try {
          retained = r.request().postDataJSON();
          const response = await r.fetch();
          expect(response.ok(), await response.text()).toBe(true);
          expect((await response.json()).status).toBe("committed");
          await r.abort("failed");
          confirm();
        } catch (error) {
          fail(error);
        }
      },
      { times: 1 },
    );
    return { committed };
  };
  const firstLost = await loseAcknowledgement();
  await revealActions();
  await row.getByRole("button", { name: "Add reaction", exact: true }).click();
  await search.fill("thumbs up");
  await picker.getByRole("button", { name: "👍", exact: true }).first().click();
  await firstLost.committed;
  await expect(reactions.getByRole("status")).toContainText(
    "Awaiting reaction confirmation",
  );
  await apply({
    command: "edit_comment",
    id: comment,
    expected_version: 1,
    expected_task_version: 2,
    body: "Edited task reaction",
  });
  await expect(row).toContainText("Edited task reaction");
  await expect(reactions.getByRole("status")).toContainText(
    "Awaiting reaction confirmation",
  );
  await reactions.getByRole("button", { name: "Check action status" }).click();
  await expect(
    reactions.getByRole("button", { name: "Check action status" }),
  ).toHaveCount(0);
  await expect(reactions.getByRole("status")).toHaveCount(0);
  expect(retained?.request_id).toBeTruthy();
  await expect(bar).toHaveCount(0);
  expect((await query()).status()).toBe(404);
  const secondLost = await loseAcknowledgement();
  await revealActions();
  await row.getByRole("button", { name: "Add reaction", exact: true }).click();
  await search.fill("thumbs up");
  await picker.getByRole("button", { name: "👍", exact: true }).first().click();
  await secondLost.committed;
  await expect(reactions.getByRole("status")).toContainText(
    "Awaiting reaction confirmation",
  );
  await apply({
    command: "tombstone_comment",
    id: comment,
    expected_version: 2,
    expected_task_version: 3,
  });
  await expect(row).toContainText("Comment removed");
  await expect(row.getByRole("button", { name: "Add reaction" })).toHaveCount(
    0,
  );
  await expect(reactions.getByRole("status")).toContainText(
    "Awaiting reaction confirmation",
  );
  await reactions.getByRole("button", { name: "Check action status" }).click();
  await expect(
    reactions.getByRole("button", { name: "Check action status" }),
  ).toHaveCount(0);
  await expect(reactions.getByRole("status")).toHaveCount(0);
  expect(
    (
      await new AxeBuilder({ page })
        .include('[aria-label="Task details"]')
        .analyze()
    ).violations,
  ).toEqual([]);
  expect(errors).toEqual([]);
  await observer.close();
});
