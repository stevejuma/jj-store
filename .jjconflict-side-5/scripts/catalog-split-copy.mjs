import { expect } from "@playwright/test";

// Native pointer/touch events are necessary: synthetic Storybook hover events
// do not activate CSS pseudo-classes. Run against the freshly built catalog.
export async function expectSplitCopy(browser, origin) {
  const git = `565656b3e4a1${"a".repeat(26)}c9`;
  const jj = `ututut${"v".repeat(24)}ut`;
  const url = `${origin}/iframe.html?id=jj-store-components-native-ids--default&viewMode=story`;
  for (const touch of [false, true]) {
    const context = await browser.newContext({
      viewport: { width: touch ? 390 : 1026, height: 844 },
      hasTouch: touch,
      isMobile: touch,
      permissions: ["clipboard-read", "clipboard-write"],
    });
    try {
      const page = await context.newPage();
      await page.goto(url);
      const group = page.getByRole("group", {
        name: "Native revision identifiers",
      });
      const commit = page.getByRole("button", {
        name: `Copy Git commit SHA ${git}`,
        exact: true,
      });
      const change = page.getByRole("button", {
        name: `Copy JJ Change ID ${jj}`,
        exact: true,
      });
      await expect(commit.getByText("5656", { exact: true })).toBeVisible();
      await expect(change.getByText("utut", { exact: true })).toBeVisible();
      const width = (await group.boundingBox()).width;
      expect(width).toBeLessThanOrEqual(122);
      expect((await group.boundingBox()).height).toBeLessThanOrEqual(27);
      const left = (await commit.boundingBox()).width;
      const right = (await change.boundingBox()).width;
      const unchangedWidth = async () =>
        expect((await group.boundingBox()).width).toBe(width);
      const noClipping = async () => {
        await expect
          .poll(() => group.evaluate((el) => el.getAnimations().length))
          .toBe(0);
        for (const button of [commit, change]) {
          const box = await button.boundingBox();
          for (const content of await button
            .locator(".community-chip, code:visible, .id-feedback:visible")
            .all()) {
            const bounds = await content.boundingBox();
            expect(bounds.x).toBeGreaterThanOrEqual(box.x + 4);
            expect(bounds.x + bounds.width).toBeLessThanOrEqual(
              box.x + box.width - 4,
            );
          }
        }
      };
      await noClipping();
      if (!touch) {
        await expect(group).toHaveCSS("transition-duration", "0.16s");
        await commit.hover();
        await expect(
          commit.getByText("56565..c9", { exact: true }),
        ).toBeVisible();
        await expect(change.getByText("JJ", { exact: true })).toBeVisible();
        await expect
          .poll(async () => (await commit.boundingBox()).width)
          .toBeGreaterThan(left + 10);
        await unchangedWidth();
        await expect
          .poll(
            async () => await group.evaluate((el) => el.getAnimations().length),
          )
          .toBe(0);
        await noClipping();
        await change.hover();
        await expect(
          change.getByText("ututu..ut", { exact: true }),
        ).toBeVisible();
        await expect(commit.getByText("GIT", { exact: true })).toBeVisible();
        await expect
          .poll(async () => (await change.boundingBox()).width)
          .toBeGreaterThan(right + 10);
        await unchangedWidth();
        await expect
          .poll(
            async () => await group.evaluate((el) => el.getAnimations().length),
          )
          .toBe(0);
        await noClipping();
        await page.emulateMedia({ reducedMotion: "reduce" });
        await expect(group).toHaveCSS("transition-duration", "0s");
        await commit.hover();
        await unchangedWidth();
        await noClipping();
        await commit.click();
      } else {
        await commit.tap();
      }
      await expect
        .poll(() => page.evaluate(() => navigator.clipboard.readText()))
        .toBe(git);
      await expect(commit.getByText("Copied", { exact: true })).toBeVisible();
      await expect(commit.getByText("GIT", { exact: true })).toBeVisible();
      await expect(
        commit.locator('[data-copy-feedback="copied"] svg'),
      ).toBeVisible();
      if (touch) await change.tap();
      else await change.click();
      await expect
        .poll(() => page.evaluate(() => navigator.clipboard.readText()))
        .toBe(jj);
      await expect(change.getByText("Copied", { exact: true })).toBeVisible();
      await expect(change.getByText("JJ", { exact: true })).toBeVisible();
      await expect(
        change.locator('[data-copy-feedback="copied"] svg'),
      ).toBeVisible();
      await expect(commit).toHaveAttribute("data-copied", "true");
      await unchangedWidth();
      await noClipping();
      await expect(commit).not.toHaveAttribute("data-copied", {
        timeout: 2000,
      });
      await expect(change).not.toHaveAttribute("data-copied", {
        timeout: 2000,
      });
      if (touch) {
        await expect(commit.getByText("5656", { exact: true })).toBeVisible();
        await expect(change.getByText("utut", { exact: true })).toBeVisible();
      }
      await page.screenshot({
        path: `.scratch/split-copy-${touch ? "touch" : "desktop"}.png`,
      });
    } finally {
      await context.close();
    }
  }
}
