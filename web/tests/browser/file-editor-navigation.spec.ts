import { expect, test } from "@playwright/test";
import { api, commit, operation, repo } from "./native-browser-fixture.js";

for (const navigation of ["breadcrumb", "history"] as const) {
  test(`returning to root through ${navigation} clears file editing`, async ({
    page,
  }) => {
    const requests = await api(page, {
      access: "local_trusted",
      editFile: true,
    });
    await page.goto(
      `/#/repos/${repo}/files?at_op=${operation}&bookmark=main&commit=${commit}`,
    );
    for (const edit of [false, true]) {
      await page.getByRole("button", { name: "Open file bin" }).click();
      if (edit) {
        await page.getByRole("button", { name: "Edit", exact: true }).click();
        await expect(page.getByLabel("File name")).toHaveValue("bin");
        const scroller = page.locator(
          ".store-file-editor__contents .cm-scroller",
        );
        const gutter = page.locator(".store-file-editor__contents .cm-gutters");
        const contents = page.getByRole("textbox", {
          name: "Editing bin file contents",
        });
        const scrollerBounds = (await scroller.boundingBox())!;
        const gutterBounds = (await gutter.boundingBox())!;
        const contentBounds = (await contents.boundingBox())!;
        expect(gutterBounds.x - scrollerBounds.x).toBeLessThanOrEqual(16);
        expect(
          scrollerBounds.x +
            scrollerBounds.width -
            contentBounds.x -
            contentBounds.width,
        ).toBeLessThanOrEqual(16);
        expect(contentBounds.width).toBeGreaterThan(scrollerBounds.width - 100);
        expect(
          await scroller.evaluate((el) => el.scrollWidth - el.clientWidth),
        ).toBeLessThanOrEqual(0);
        await expect(
          page.getByRole("combobox", { name: "Repository workspace" }),
        ).toBeDisabled();
      } else await expect(page.getByLabel("File name")).toHaveCount(0);
      const navigate = async () => {
        if (navigation === "breadcrumb")
          await page.getByRole("link", { name: "Files", exact: true }).click();
        else await page.goBack();
      };
      await navigate();
      if (edit) {
        await expect(page.getByLabel("File name")).toHaveValue("bin");
        await expect(page).toHaveURL(/file=bin/);
        await expect(
          page.getByText(
            "Finish the current repository action before navigating.",
            { exact: true },
          ),
        ).toBeVisible();
        await page.getByRole("button", { name: "Cancel", exact: true }).click();
        await navigate();
      }
      await expect(
        page.getByRole("table", { name: "Repository files" }),
      ).toBeVisible();
      await expect(page.getByLabel("File name")).toHaveCount(0);
      await expect(
        page.getByRole("combobox", { name: "Repository workspace" }),
      ).toBeEnabled();
      expect(new URL(page.url()).hash).not.toContain("file=");
    }
    expect(requests.some((path) => path.startsWith("mutations"))).toBe(false);
  });
}
