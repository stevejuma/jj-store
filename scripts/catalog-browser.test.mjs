import test from "node:test";
import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile, mkdir } from "node:fs/promises";
import { resolve, extname, sep } from "node:path";
import { chromium, expect } from "@playwright/test";

async function expectSettledGraph(page) {
  await expect
    .poll(
      () =>
        page.locator('[data-ui-part="virtual-row"]').evaluateAll((rows) => {
          const issues = [];
          for (let index = 0; index < rows.length; index++) {
            const row = rows[index];
            const bounds = row.getBoundingClientRect();
            const next = rows[index + 1]?.getBoundingClientRect();
            if (next && bounds.bottom > next.top + 1)
              issues.push(
                `row ${index} overlaps next by ${bounds.bottom - next.top}px`,
              );
            const content = row.querySelector(".community-revision-graph-copy");
            const line = row.querySelector(".community-revision-graph__row");
            if (content && line) {
              const body = content.getBoundingClientRect();
              const frame = line.getBoundingClientRect();
              if (body.top < frame.top - 1 || body.bottom > frame.bottom + 1)
                issues.push(`row ${index} content exceeds line height`);
              const ids = row
                .querySelector(".community-revision-graph-ids")
                ?.getBoundingClientRect();
              for (const chip of content.querySelectorAll(
                '[data-ui-part="community-chip"]',
              )) {
                const box = chip.getBoundingClientRect();
                if (
                  ids &&
                  box.right > ids.left + 1 &&
                  box.left < ids.right &&
                  box.bottom > ids.top &&
                  box.top < ids.bottom
                )
                  issues.push(`row ${index} bookmark overlaps IDs`);
              }
            }
          }
          return issues;
        }),
      {
        message: "Measured graph rows and expanded files must not overlap",
        timeout: 5000,
      },
    )
    .toEqual([]);
}

// Cold, loopback-only static deployment acceptance, separate from visual approval.
test("cold_catalog_contract", { timeout: 60000 }, async () => {
  const root = resolve("storybook-static");
  const types = {
    ".html": "text/html",
    ".js": "text/javascript",
    ".css": "text/css",
    ".json": "application/json",
    ".svg": "image/svg+xml",
    ".woff2": "font/woff2",
  };
  const server = createServer(async (request, response) => {
    try {
      const url = new URL(request.url, "http://localhost");
      const path = resolve(
        root,
        `.${decodeURIComponent(url.pathname === "/" ? "/index.html" : url.pathname)}`,
      );
      if (!path.startsWith(`${root}${sep}`)) {
        response.writeHead(403).end();
        return;
      }
      const data = await readFile(path);
      response
        .writeHead(200, {
          "content-type": types[extname(path)] ?? "application/octet-stream",
        })
        .end(data);
    } catch {
      response.writeHead(404).end();
    }
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const origin = `http://127.0.0.1:${server.address().port}`;
  let browser;
  const unexpected = [],
    errors = [];
  try {
    browser = await chromium.launch();
    const context = await browser.newContext({
      viewport: { width: 1440, height: 1000 },
      reducedMotion: "reduce",
    });
    await context.route("**/*", async (route) => {
      const url = new URL(route.request().url());
      if (url.origin !== origin || url.pathname.startsWith("/v1/")) {
        unexpected.push(url.href);
        await route.abort();
      } else await route.continue();
    });
    const page = await context.newPage();
    page.setDefaultTimeout(10000);
    page.on("pageerror", (error) => errors.push(error.message));
    page.on("websocket", (socket) => unexpected.push(socket.url()));
    const url = `${origin}/iframe.html?id=jj-store-application--expanded-workspace&viewMode=story`;
    await page.goto(url);
    await expect(
      page.getByRole("button", {
        name: "Collapse Branches column",
        exact: true,
      }),
    ).toBeVisible();
    await expect(
      page.locator('[data-ui-component="community-app-shell"]'),
    ).toHaveCSS("font-family", /DM Sans Variable/);
    const iconSizes = await page
      .locator('[aria-label="Application navigation"] button svg')
      .evaluateAll((icons) =>
        icons.map((icon) => icon.getBoundingClientRect().width),
      );
    assert.ok(
      iconSizes.every((size) => size > 0 && size <= 16),
      `Header icon widths: ${iconSizes}`,
    );
    const search = page.getByPlaceholder("Search branches", { exact: true });
    await search.fill("main");
    await search.press("ArrowDown");
    await search.press("Enter");
    const branches = page.locator(
      '[data-ui-part="column"][data-column-id="branches"]',
    );
    const resize = branches.getByRole("separator");
    const width = Number(await resize.getAttribute("aria-valuenow"));
    await resize.focus();
    await resize.press("ArrowRight");
    await expect
      .poll(async () => Number(await resize.getAttribute("aria-valuenow")))
      .toBeGreaterThan(width);
    await page
      .getByRole("button", { name: "Collapse Branches column" })
      .click();
    await page.getByRole("button", { name: "Expand Branches column" }).click();
    await expect(search).toBeVisible();
    await page
      .getByRole("button", {
        name: "Merge native storage and publication",
        exact: true,
      })
      .click();
    await expectSettledGraph(page);
    await page.getByRole("radio", { name: "Show files as list" }).click();
    const file = page.getByRole("button", { name: "src/lib.rs", exact: true });
    await file.focus();
    await file.press("Enter");
    await expect(page.locator('[data-ui-component="file-diff"]')).toContainText(
      "publish_native_objects()?;",
    );
    const shell = page.locator('[data-ui-component="community-app-shell"]');
    assert.equal(await shell.getAttribute("data-display-mode"), "desktop");
    await mkdir(".scratch", { recursive: true });
    await page.screenshot({ path: ".scratch/catalog-desktop.png" });
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto(url);
    await expect(shell).toHaveAttribute("data-display-mode", "mobile");
    await page.getByRole("button", { name: "Open navigation sidebar" }).click();
    await page.getByRole("button", { name: "JJ Log", exact: true }).click();
    await expect(
      page.getByRole("button", { name: "Collapse JJ Log column", exact: true }),
    ).toBeVisible();
    await page
      .getByRole("button", {
        name: "Merge native storage and publication",
        exact: true,
      })
      .click();
    await expect(
      page.getByRole("region", { name: "Changed files", exact: true }),
    ).toBeVisible();
    await expectSettledGraph(page);
    await page.screenshot({ path: ".scratch/catalog-mobile.png" });
    // A fresh instance must not inherit page, bookmark or sidebar state.
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.reload();
    await expect(
      page.getByRole("button", {
        name: "Collapse Branches column",
        exact: true,
      }),
    ).toBeVisible();
    await expect(
      page.getByRole("region", { name: "Changed files", exact: true }),
    ).toHaveCount(0);
    await page.goto(
      `${origin}/iframe.html?id=jj-store-application--docs&viewMode=docs`,
    );
    // Autodocs renders a primary preview plus each of the four named stories.
    await expect(page.locator("iframe[title]")).toHaveCount(5);
    for (const frame of await page.locator("iframe[title]").all()) {
      const bounds = await frame.boundingBox();
      assert.ok(
        bounds && bounds.height >= 700,
        "Shell documentation needs a bounded application canvas",
      );
    }
    await page.goto(url);
    await page.getByRole("button", { name: "jj-store", exact: true }).click();
    await page.getByRole("button", { name: "README.md", exact: true }).click();
    const code = page.locator('[data-ui-component="code-block"]').first();
    await expect(code).toContainText("# jj-store");
    await expect(code).toHaveCSS("font-family", /Source Code Pro/);
    await page.screenshot({ path: ".scratch/catalog-files.png" });
    await page.setViewportSize({ width: 390, height: 844 });
    await expect(code).toBeVisible();
    await page.screenshot({ path: ".scratch/catalog-files-mobile.png" });
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.goto(
      `${origin}/iframe.html?id=jj-store-application--full-app&viewMode=story`,
    );
    await expect(
      page.getByRole("button", {
        name: "Collapse Branches column",
        exact: true,
      }),
    ).toBeVisible();
    await expect(
      page.getByRole("option", { name: "main", exact: true }),
    ).toContainText("just now by Maya Chen");
    await page.screenshot({ path: ".scratch/catalog-full-app.png" });
    assert.deepEqual(unexpected, []);
    assert.deepEqual(errors, []);
  } finally {
    await browser?.close();
    await new Promise((resolve) => server.close(resolve));
  }
});
