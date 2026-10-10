import { expect, test, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { api, commit, operation, repo } from "./native-browser-fixture.js";

const manifest = JSON.parse(
  readFileSync(
    new URL("../../dist/.vite/manifest.json", import.meta.url),
    "utf8",
  ),
);
const chunk = (name: string): string =>
  manifest[
    Object.keys(manifest).find((key) => key.endsWith(`/${name}.svelte`))!
  ].file;

async function settings(page: Page) {
  const expand = page.getByRole("button", {
    name: "Expand left sidebar",
    exact: true,
  });
  if (await expand.isVisible()) await expand.click();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
}

test("a failed view chunk offers explicit retry without reloading or replaying commands", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  let imports = 0;
  let mutations = 0;
  page.on("request", (request) => {
    if (request.method() === "POST") mutations += 1;
  });
  await page.route(`**/${chunk("StoreSettings")}*`, async (route) => {
    if (++imports === 1)
      await route.fulfill({ status: 503, body: "temporarily unavailable" });
    else await route.continue();
  });
  await page.goto(`/#/repos/${repo}/files?at_op=${operation}`);
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await settings(page);
  await expect(page.getByText("The view could not be loaded.")).toBeVisible();
  await page.getByRole("button", { name: "Retry", exact: true }).click();
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
  expect(imports).toBe(2);
  expect(mutations).toBe(0);
  expect(new URL(page.url()).hash).toContain(`at_op=${operation}`);
});

test("cold built shell loads selected views on demand and reuses completed imports", async ({
  page,
}) => {
  const requests: string[] = [];
  page.on("request", (request) =>
    requests.push(new URL(request.url()).pathname),
  );
  await api(page, { access: "local_trusted" });
  const opened = Date.now();
  await page.goto(`/#/repos/${repo}/files?at_op=${operation}`);
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  expect(requests.some((path) => /\.svelte(?:\.ts)?$/.test(path))).toBe(false);
  console.log(
    "Cold built Files measurement",
    JSON.stringify({
      requests: requests.length,
      renderedMillis: Date.now() - opened,
      timing: await page.evaluate(() => {
        const navigation = performance.getEntriesByType(
          "navigation",
        )[0] as PerformanceNavigationTiming;
        return {
          domContentLoaded: navigation.domContentLoadedEventEnd,
          load: navigation.loadEventEnd,
          paints: performance
            .getEntriesByType("paint")
            .map(({ name, startTime }) => ({ name, startTime })),
        };
      }),
    }),
  );
  for (const view of [
    "StoreSettings",
    "StoreReviewThreads",
    "StoreAgentExecutionHost",
    "StoreProjects",
  ])
    expect(requests).not.toContain(`/${chunk(view)}`);
  await settings(page);
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
  expect(
    requests.filter((path) => path === `/${chunk("StoreSettings")}`),
  ).toHaveLength(1);
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  await settings(page);
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
  expect(
    requests.filter((path) => path === `/${chunk("StoreSettings")}`),
  ).toHaveLength(1);
});

test("navigation away from a delayed view cannot render its obsolete completion", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(`**/${chunk("StoreSettings")}`, async (route) => {
    await pending;
    await route.continue();
  });
  await page.goto(`/#/repos/${repo}/files?at_op=${operation}`);
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await settings(page);
  await expect(
    page.getByRole("status", { name: "Loading Settings", exact: true }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  release();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toHaveCount(0);
  await settings(page);
  await expect(
    page.getByRole("region", { name: "Settings content" }),
  ).toBeVisible();
});

test("cold Threads import preserves the inbox columns before the view arrives", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(`**/${chunk("StoreThreads")}`, async (route) => {
    await pending;
    await route.continue();
  });
  try {
    await page.goto("/#/threads");
    await expect(
      page.getByRole("status", { name: "Loading Threads", exact: true }),
    ).toBeVisible();
    const list = page.getByRole("complementary", {
      name: "Thread conversations",
      exact: true,
    });
    await expect(list).toBeVisible();
    const body = page.getByRole("region", {
      name: "Selected thread",
      exact: true,
    });
    await expect(body).toBeVisible();
    const left = await list.boundingBox();
    const right = await body.boundingBox();
    expect(right!.x).toBeGreaterThanOrEqual(left!.x + left!.width - 1);
    expect(right!.y).toBeCloseTo(left!.y, 0);
  } finally {
    release();
  }
});

for (const view of [
  {
    url: `/#/repos/${repo}/log?at_op=${operation}&commit=${commit}&file=bin`,
    module: "CommunityProjectsRepositoryLog",
    label: "History",
    selector: '[data-layout="diff"]',
  },
  {
    url: `/#/repos/${repo}/files?at_op=${operation}&commit=${commit}&file=README.md`,
    module: "CommunityProjectsRepositoryFiles",
    label: "Files",
    selector: '[data-layout="source"]',
  },
  {
    url: `/#/repos/${repo}/reviews?at_op=${operation}&thread=${"23".repeat(16)}`,
    module: "StoreReviewThreads",
    label: "Reviews",
    selector: '[data-layout="review-detail"]',
  },
  {
    url: `/#/agents/sessions/${"23".repeat(16)}`,
    module: "StoreAgentExecutionHost",
    label: "Agents",
    selector: '[data-layout="messages"]',
  },
  {
    url: `/#/threads/${repo}/${"23".repeat(16)}`,
    module: "StoreThreads",
    label: "Threads",
    selector: '[data-layout="messages"]',
  },
  {
    url: "/#/agents/sessions",
    module: "StoreAgentExecutionHost",
    label: "Agents",
    selector: ".lc-community-inbox",
    regions: ["Session chats", "Selected session"],
  },
  {
    url: "/#/agents",
    module: "StoreAgentExecutionHost",
    label: "Agents",
    selector: '[data-layout="gallery"][data-gallery-variant="agents"]',
  },
  ...["branches", "log", "oplog", "files", "agents", "reviews", "projects"].map(
    (tab) => ({
      url: `/#/repos/${repo}/${tab}?at_op=${operation}`,
      module:
        tab === "files"
          ? "CommunityProjectsRepositoryFiles"
          : tab === "agents"
            ? "StoreAgentSessions"
            : tab === "reviews"
              ? "StoreReviewThreads"
              : tab === "projects"
                ? "StoreRepositoryProjects"
                : "CommunityProjectsRepositoryLog",
      label:
        tab === "files"
          ? "Files"
          : tab === "agents"
            ? "Agent sessions"
            : tab === "reviews"
              ? "Reviews"
              : tab === "projects"
                ? "Repository projects"
                : "History",
      selector: `[data-layout="${tab === "branches" ? "bookmarks" : tab === "log" || tab === "oplog" ? "graph" : tab === "files" ? "directory" : tab === "agents" ? "" : "work-list"}"]`,
      tab,
    }),
  ),
  ...["projects", "tasks", "reviews", "agents", "repositories", "activity"].map(
    (tab) => ({
      url: `/#/projects?tab=${tab}`,
      module: "StoreProjects",
      label: "Projects",
      selector:
        tab === "agents"
          ? '.store-collaboration-skeleton[data-kind="agent"]'
          : `[data-layout="${tab === "activity" ? "timeline" : "work-list"}"]`,
      tab,
    }),
  ),
]) {
  test(`cold ${view.url} uses its page skeleton while its module is delayed`, async ({
    page,
  }) => {
    await api(page, { access: "local_trusted" });
    let release!: () => void;
    const pending = new Promise<void>((resolve) => {
      release = resolve;
    });
    await page.route(`**/${chunk(view.module)}`, async (route) => {
      await pending;
      await route.continue();
    });
    try {
      await page.goto(view.url);
      const status = page.getByRole("status", {
        name: `Loading ${view.label}`,
        exact: true,
      });
      await expect(status).toBeVisible();
      const selector =
        "tab" in view &&
        view.tab === "agents" &&
        view.module === "StoreAgentSessions"
          ? '.store-collaboration-skeleton[data-kind="agent"]'
          : view.selector;
      await expect(status.locator(selector)).toBeVisible();
      await expect(
        status.locator('.community-skeleton[data-layout="details"]'),
      ).toHaveCount(0);
      if ("regions" in view) {
        const left = await page
          .getByRole("complementary", { name: view.regions![0], exact: true })
          .boundingBox();
        const right = await page
          .getByRole("region", { name: view.regions![1], exact: true })
          .boundingBox();
        expect(right!.x).toBeGreaterThanOrEqual(left!.x + left!.width - 1);
        expect(left!.height).toBeGreaterThan(500);
      }
    } finally {
      release();
    }
  });
}

test("Files deep-link keeps a source skeleton through the initial repository read", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(`**/v1/repos/${repo}/snapshot*`, async (route) => {
    await pending;
    await route.fallback();
  });
  try {
    await page.goto(
      `/#/repos/${repo}/files?at_op=${operation}&commit=${commit}&file=README.md`,
    );
    await expect(
      page.locator(
        '[aria-label="Loading repository files"][data-layout="source"]',
      ),
    ).toBeVisible();
    await expect(page.locator('[data-layout="directory"]')).toHaveCount(0);
  } finally {
    release();
  }
});

test("saved inbox width survives the module-to-page transition", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.addInitScript(() =>
    localStorage.setItem(
      "jj-store/layout/v1/agent-inbox/sessions",
      JSON.stringify({ version: 1, panel: "agent-sessions", width: 240 }),
    ),
  );
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(`**/${chunk("StoreThreads")}`, async (route) => {
    await pending;
    await route.continue();
  });
  await page.goto("/#/threads");
  const list = page.getByRole("complementary", {
    name: "Thread conversations",
    exact: true,
  });
  try {
    await expect(
      page.getByRole("status", { name: "Loading Threads", exact: true }),
    ).toBeVisible();
    expect((await list.boundingBox())!.width).toBe(240);
  } finally {
    release();
  }
  await expect(
    page.getByRole("status", { name: "Loading Threads", exact: true }),
  ).toHaveCount(0);
  expect((await list.boundingBox())!.width).toBe(240);
});

test("saved open file tree is represented before the Files module loads", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.addInitScript(() =>
    localStorage.setItem(
      "jj-store/layout/v1/file-tree",
      JSON.stringify({ version: 1, open: true, width: 300 }),
    ),
  );
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(
    `**/${chunk("CommunityProjectsRepositoryFiles")}`,
    async (route) => {
      await pending;
      await route.continue();
    },
  );
  try {
    await page.goto(`/#/repos/${repo}/files?at_op=${operation}`);
    await expect(
      page.getByRole("status", { name: "Loading Files", exact: true }),
    ).toBeVisible();
    const tree = page.getByRole("complementary", {
      name: "Repository file tree",
      exact: true,
    });
    await expect(tree).toBeVisible();
    expect((await tree.boundingBox())!.width).toBe(300);
  } finally {
    release();
  }
});

test("mobile Threads loading keeps the inbox regions within the viewport", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await api(page, { access: "local_trusted" });
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route(`**/${chunk("StoreThreads")}`, async (route) => {
    await pending;
    await route.continue();
  });
  try {
    await page.goto("/#/threads");
    await expect(
      page.getByRole("status", { name: "Loading Threads", exact: true }),
    ).toBeVisible();
    const content = page.getByRole("region", {
      name: "Selected thread",
      exact: true,
    });
    await expect(content).toBeVisible();
    expect((await content.boundingBox())!.width).toBeLessThanOrEqual(390);
    await page
      .getByRole("button", { name: "Toggle thread conversations", exact: true })
      .click();
    const list = page.getByRole("complementary", {
      name: "Thread conversations",
      exact: true,
    });
    await expect(list).toBeVisible();
    expect((await list.boundingBox())!.width).toBeLessThanOrEqual(390);
  } finally {
    release();
  }
});
