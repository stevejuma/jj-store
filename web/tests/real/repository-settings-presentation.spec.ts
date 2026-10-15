import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const repository = process.env.JJ_STORE_REAL_REPOSITORY!;

for (const narrow of [false, true]) {
  test(`repository Settings cards and secondary tabs at ${narrow ? "mobile" : "desktop"} width`, async ({
    page,
  }) => {
    test.setTimeout(120000);
    await page.setViewportSize(
      narrow ? { width: 390, height: 844 } : { width: 1351, height: 988 },
    );
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    for (const destination of [
      "general",
      "advanced",
      "storage&tab=overview",
      "sync&tab=configuration",
      "search&tab=configuration",
      "search&tab=scopes",
      "search&tab=advanced",
    ]) {
      await page.emulateMedia({
        colorScheme: destination === "advanced" ? "dark" : "light",
      });
      await page.goto(`/#/repos/${repository}/settings?section=${destination}`);
      await expect(
        page.locator(".store-settings-card:visible").first(),
      ).toBeVisible();
      await page.evaluate(() => document.fonts.ready);
      const cards = page.locator(".store-settings-card:visible");
      for (const card of await cards.all()) {
        const header = card.locator(".store-settings-card__header");
        await expect(header).toBeVisible();
        expect(
          await header.evaluate((e) => getComputedStyle(e).backgroundColor),
        ).not.toBe("rgba(0, 0, 0, 0)");
        expect(
          await card.evaluate((e) => {
            const header = e.querySelector(".store-settings-card__header")!;
            const content = e.querySelector(".store-settings-card__content")!;
            return Math.abs(
              content.getBoundingClientRect().top -
                header.getBoundingClientRect().bottom,
            );
          }),
        ).toBeLessThanOrEqual(1);
        for (const row of await card.locator(".store-readonly-row").all()) {
          const bounds = (await row.boundingBox())!,
            outer = (await card.boundingBox())!;
          expect(Math.abs(bounds.x - outer.x)).toBeLessThanOrEqual(2);
          expect(Math.abs(bounds.width - outer.width)).toBeLessThanOrEqual(2);
        }
      }
      if (destination !== "advanced" && destination !== "general") {
        const strip = page.getByRole("navigation", {
          name: "Settings page navigation",
        });
        const active = strip
          .locator(
            '[role="tab"][aria-selected="true"], .store-application__tab-more[data-selected]',
          )
          .filter({ visible: true });
        await expect(active).toHaveCSS("font-weight", "600");
        expect(
          await active.evaluate((e) => getComputedStyle(e, "::after").height),
        ).toBe("3px");
        expect(
          await active.evaluate((e) => getComputedStyle(e, "::after").opacity),
        ).toBe("1");
        expect(
          await active.evaluate((e) => {
            const style = getComputedStyle(e),
              indicator = getComputedStyle(e, "::after");
            const bottom =
              e.getBoundingClientRect().bottom -
              parseFloat(style.borderBottomWidth) -
              parseFloat(indicator.bottom);
            return Math.abs(
              bottom -
                e
                  .closest(".store-application__tab-space")!
                  .getBoundingClientRect().bottom,
            );
          }),
        ).toBeLessThanOrEqual(1);
        const list = strip.getByRole("tablist"),
          first = list.getByRole("tab").first();
        const firstBounds = (await first.boundingBox())!,
          listBounds = (await list.boundingBox())!;
        expect(firstBounds.width).toBeLessThan(listBounds.width);
        expect(firstBounds.x - listBounds.x).toBeLessThanOrEqual(4);
      }
      if (destination === "general") {
        for (const row of await page
          .getByRole("region", { name: "Identity and snapshot" })
          .locator(".store-readonly-row")
          .all()) {
          await expect(row.locator("dt svg")).toHaveCount(1);
          if (await row.locator(".store-readonly-row__copy").count()) {
            const value = row.locator(".store-readonly-row__value");
            await expect(value).toHaveCSS("white-space", "nowrap");
            await expect(value).toHaveCSS("text-overflow", "ellipsis");
            expect(
              (await row.locator(".store-readonly-row__copy").boundingBox())!
                .height,
            ).toBeLessThanOrEqual(24);
          }
        }
      }
      if (destination === "search&tab=advanced") {
        const primary = page.getByRole("navigation", {
          name: "Repository settings navigation",
          exact: true,
        });
        const secondary = page.getByRole("navigation", {
          name: "Settings page navigation",
          exact: true,
        });
        const before = [
          (await primary.boundingBox())!.y,
          (await secondary.boundingBox())!.y,
        ];
        expect(
          (await secondary.boundingBox())!.y -
            ((await primary.boundingBox())!.y +
              (await primary.boundingBox())!.height),
        ).toBeLessThanOrEqual(1);
        const viewport = page.locator(
          '.store-settings__scroll:visible [data-slot="scroll-area-viewport"]',
        );
        await viewport.focus();
        await viewport.press("End");
        await expect
          .poll(() => viewport.evaluate((e) => e.scrollTop))
          .toBeGreaterThan(0);
        expect([
          (await primary.boundingBox())!.y,
          (await secondary.boundingBox())!.y,
        ]).toEqual(before);
        await expect(page.locator(".store-repository-settings-host")).toHaveCSS(
          "overflow",
          "hidden",
        );
      }
      for (const footer of await page
        .locator(".store-settings-footer:visible")
        .all()) {
        const actions = (await footer
          .locator(".store-settings__actions")
          .boundingBox())!;
        const bounds = (await footer.boundingBox())!;
        expect(
          bounds.x + bounds.width - actions.x - actions.width,
        ).toBeLessThanOrEqual(18);
        const description = footer.locator(
          ".store-settings-footer__description",
        );
        if (await description.count()) {
          const text = (await description.boundingBox())!;
          if (narrow)
            expect(actions.y).toBeGreaterThanOrEqual(text.y + text.height);
          else expect(text.x + text.width).toBeLessThanOrEqual(actions.x);
        }
      }
      await expect(
        page.locator(
          ".store-settings--repository:visible dl button:visible:not(.store-readonly-row__copy):not(.store-readonly-row__open)",
        ),
      ).toHaveCount(0);
      for (const input of await page
        .locator(
          '.store-settings--repository:visible input[type="number"]:visible',
        )
        .all()) {
        const joined = input.locator(
          "xpath=ancestor::*[contains(@class,'store-numeric-setting__input')]",
        );
        if (await joined.count()) {
          const entry = (await joined.boundingBox())!;
          const control = (await input.boundingBox())!;
          expect(control.x).toBeGreaterThanOrEqual(entry.x);
          expect(control.x + control.width).toBeLessThanOrEqual(
            entry.x + entry.width,
          );
          if (!narrow) expect(entry.width).toBeLessThanOrEqual(248);
        } else
          expect((await input.boundingBox())!.width).toBeLessThanOrEqual(180);
      }
      const save = page.getByRole("button", {
        name: "Save settings",
        exact: true,
      });
      if (await save.count())
        expect(
          await save.evaluate((e) => e.parentElement?.lastElementChild === e),
        ).toBe(true);
      expect(
        await page.evaluate(
          () => document.documentElement.scrollWidth <= innerWidth,
        ),
      ).toBe(true);
      expect(
        (
          await new AxeBuilder({ page })
            .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
            .analyze()
        ).violations,
      ).toEqual([]);
    }
    expect(errors).toEqual([]);
  });
}
