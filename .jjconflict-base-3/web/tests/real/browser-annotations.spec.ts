import { expect, test } from "@playwright/test";
import { execFileSync } from "node:child_process";

test("browser annotation helpers render only with the explicit development policy", async ({
  page,
  request,
}) => {
  test.setTimeout(180000);
  const enabled = process.env.JJ_STORE_DEV_BROWSER_ANNOTATIONS === "1";
  const home = process.env.JJ_STORE_REAL_REPOSITORY!;
  const origin = new URL(process.env.JJ_STORE_REAL_BASE_URL!).origin;
  const headers = {
    Origin: origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  async function workSnapshot() {
    const response = await request.get(`/v1/repos/${home}/work`, { headers });
    expect(response.ok()).toBe(true);
    expect(response.headers()["content-security-policy"]).not.toContain(
      "unsafe-inline",
    );
    return (await response.json()).snapshot;
  }
  async function probe() {
    return page.evaluate(() => {
      const overlay = document.createElement("div");
      overlay.id = "browser-annotation-policy-probe";
      overlay.textContent = "Annotation overlay probe";
      document.body.append(overlay);
      const style = document.createElement("style");
      style.textContent =
        "#browser-annotation-policy-probe { position: fixed; top: 10px; right: 10px; padding: 12px; z-index: 2147483647; background: rgb(10, 70, 130); color: white; }";
      document.head.append(style);
      const script = document.createElement("script");
      script.textContent =
        "document.documentElement.setAttribute('data-annotation-probe', 'ready')";
      document.head.append(script);
      return {
        inlineScriptRan:
          document.documentElement.getAttribute("data-annotation-probe") ===
          "ready",
        overlayStyled: getComputedStyle(overlay).position === "fixed",
      };
    });
  }
  const before = await workSnapshot();
  const loaded = await page.goto("/#/projects?tab=reviews");
  expect(
    loaded?.headers()["content-security-policy"].includes("unsafe-inline"),
  ).toBe(enabled);
  await expect(
    page.getByRole("region", { name: "Projects and tasks" }),
  ).toBeVisible();
  expect(await probe()).toEqual({
    inlineScriptRan: enabled,
    overlayStyled: enabled,
  });
  if (enabled) {
    await expect(
      page.locator("#browser-annotation-policy-probe"),
    ).toBeVisible();
    await page.screenshot({
      path: test.info().outputPath("annotation-overlay.png"),
    });
  }
  execFileSync("pnpm", ["dev:restart"], {
    cwd: process.env.JJ_STORE_REAL_ROOT!,
    encoding: "utf8",
    env: {
      ...process.env,
      JJ_STORE_DEV_ROOT: process.env.JJ_STORE_REAL_RUNTIME!,
      JJ_STORE_DEV_PORT: new URL(origin).port,
      JJ_STORE_DEV_BROWSER_ANNOTATIONS: "0",
    },
  });
  const strict = await page.reload();
  expect(strict?.headers()["content-security-policy"]).not.toContain(
    "unsafe-inline",
  );
  await expect(
    page.getByRole("region", { name: "Projects and tasks" }),
  ).toBeVisible();
  expect(await probe()).toEqual({
    inlineScriptRan: false,
    overlayStyled: false,
  });
  expect(await workSnapshot()).toEqual(before);
});
