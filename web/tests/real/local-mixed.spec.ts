import { expect, test } from "@playwright/test";
import { readFileSync } from "node:fs";
const runtime = process.env.JJ_STORE_REAL_RUNTIME;
const fixture = runtime
  ? JSON.parse(readFileSync(`${runtime}/runtime.json`, "utf8"))
  : null;
test.skip(!fixture, "owned mixed runtime required");
test("developer_experience_opens_linked_git_repository_and_preserves_mixed_navigation", async ({
  page,
  request,
}) => {
  const home = fixture.repositoryId,
    local = fixture.repositoryProfiles.localGit;
  const headers = {
    Origin: new URL(fixture.origin).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const repos = await request.get("/v1/repositories?limit=100", { headers });
  expect(repos.ok()).toBe(true);
  expect(
    (await repos.json()).repositories.find((r: any) => r.id === local).profile,
  ).toBe("git_sha1_jj045");
  const work = await request.get(`/v1/repos/${home}/work?limit=256`, {
    headers,
  });
  const project = (await work.json()).projects.find(
    (p: any) => p.title === "Developer experience",
  );
  expect(project.repositories[local]).toBe(true);
  await page.goto(`/#/projects?home=${home}&project=${project.id}&tab=tasks`);
  await expect(
    page.getByRole("region", { name: "Projects and tasks" }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Expand left sidebar", exact: true })
    .click();
  const sidebar = page.getByRole("complementary", {
    name: "Repository navigation sidebar",
  });
  await sidebar
    .getByRole("button", { name: "Developer experience", exact: true })
    .click();
  await sidebar
    .getByRole("button", { name: "Local Git-backed JJ", exact: true })
    .first()
    .click();
  await expect(page).toHaveURL(new RegExp(`/repos/${local}/files`));
  await expect(
    page.getByText("Local Git-backed JJ repository", { exact: false }).first(),
  ).toBeVisible();
  expect(
    new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get("commit"),
  ).toMatch(/^[0-9a-f]{40}$/);
  await page.reload();
  await expect(
    page.getByText("Local Git-backed JJ repository", { exact: false }).first(),
  ).toBeVisible();
  expect(
    await page.evaluate(() =>
      Object.keys(localStorage).some((k) => k.includes("navigation")),
    ),
  ).toBe(true);
  await page.goto(`/#/repos/${home}/files`);
  await expect(
    page.getByText("jj-store demo", { exact: false }).first(),
  ).toBeVisible();
  await page.goBack();
  await expect(page).toHaveURL(new RegExp(`/repos/${local}/files`));
  await expect(
    page.getByText("Local Git-backed JJ repository", { exact: false }).first(),
  ).toBeVisible();
  await page.goForward();
  await expect(page).toHaveURL(new RegExp(`/repos/${home}/files`));
});
