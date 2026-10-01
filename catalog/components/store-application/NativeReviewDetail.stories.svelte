<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Native review detail",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initialPage: "reviews", initiallyCollapsed: true },
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Community sections and inline agent context"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Revision open/ }),
    );
    const detail = c.getByRole("article", { name: "Review details" });
    await expect(c.queryByRole("tab", { name: "Reviews" })).toBeNull();
    await expect(
      Number.parseFloat(
        getComputedStyle(
          detail.querySelector(".community-projects__review-body")!,
        ).paddingLeft,
      ),
    ).toBeGreaterThan(0);
    await expect(
      c.queryByRole("banner", { name: "Workspace section header" }),
    ).toBeNull();
    await expect(
      canvasElement.querySelector('[aria-label="Workspace section header"]'),
    ).toBeNull();
    const view = within(detail);
    const heading = detail
      .querySelector<HTMLElement>(".community-projects__review-header")!
      .getBoundingClientRect();
    const back = view
      .getByRole("button", { name: "Back to reviews" })
      .getBoundingClientRect();
    await expect(back.left - heading.left).toBeLessThanOrEqual(8);
    await expect(
      Math.abs(back.top + back.height / 2 - heading.top - heading.height / 2),
    ).toBeLessThanOrEqual(1);
    for (const name of [
      "Description",
      "Files changed",
      "Commits",
      "Activity",
    ]) {
      await expect(view.getByRole("button", { name })).toBeVisible();
    }
    await userEvent.click(view.getByRole("button", { name: "Files changed" }));
    await expect(
      view.getByRole("complementary", { name: "Changed files" }),
    ).toBeVisible();
    await expect(
      detail.querySelector('[data-ui-component="file-diff-composer"]'),
    ).toBeVisible();
    await userEvent.click(
      view.getAllByRole("button", { name: "Hide changed files" })[0]!,
    );
    await expect(
      view.queryByRole("complementary", { name: "Changed files" }),
    ).toBeNull();
    await userEvent.click(
      view.getAllByRole("button", { name: "Show changed files" })[0]!,
    );
    await expect(
      view.getByRole("complementary", { name: "Changed files" }),
    ).toBeVisible();
    await userEvent.click(view.getByRole("button", { name: "Commits" }));
    await expect(
      view.getByRole("list", { name: "Review commits" }),
    ).toHaveTextContent("native");
    await expect(
      view.getByRole("list", { name: "Repository activity" }),
    ).toBeVisible();
    await userEvent.click(view.getByRole("button", { name: "Agent context" }));
    await expect(
      view.getByRole("log", { name: "Agent conversation activity" }),
    ).toHaveTextContent("Updated the parser");
    await expect(
      view.getByRole("group", { name: "apply_patch tool call" }),
    ).toBeVisible();
    await expect(
      view.getByRole("list", { name: "Review comments" }),
    ).toBeVisible();
    const header = detail.querySelector<HTMLElement>(
      ".community-projects__review-header",
    )!;
    const scrollRoot = canvasElement.querySelector<HTMLElement>(
      ".store-collaboration__review-scroll",
    )!;
    scrollRoot.style.maxHeight = "24rem";
    const viewport = scrollRoot.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await waitFor(() =>
      expect(viewport.scrollHeight).toBeGreaterThan(viewport.clientHeight),
    );
    viewport.scrollTop = 0;
    const headerTop = header.getBoundingClientRect().top;
    viewport.scrollTop = 240;
    await waitFor(() => expect(viewport.scrollTop).toBeGreaterThan(0));
    await expect(header.getBoundingClientRect().top).toBeCloseTo(headerTop, 0);
    await userEvent.click(
      view.getByRole("button", { name: "Back to reviews" }),
    );
    await expect(c.getByRole("list", { name: "Review threads" })).toBeVisible();
  }}
/>

<Story
  name="Community review list opens full page detail"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const list = c.getByRole("list", { name: "Review threads" });
    await expect(list).toBeVisible();
    await expect(
      canvasElement.querySelector('[data-ui-component="column-canvas"]'),
    ).toBeNull();
    await expect(
      canvasElement.querySelector('[aria-label="Review layout"]'),
    ).toBeNull();
    await expect(
      c.getByRole("button", { name: "Refresh reviews" }),
    ).toBeVisible();
    const row = list.querySelector<HTMLElement>(
      "article.community-projects__review-row",
    )!;
    await expect(row).not.toBeNull();
    await expect(row.getBoundingClientRect().width).toBeGreaterThan(600);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Revision open/ }),
    );
    await expect(c.queryByRole("list", { name: "Review threads" })).toBeNull();
    await expect(c.queryByRole("tab", { name: "Reviews" })).toBeNull();
    await expect(
      c.getByRole("article", { name: "Review details" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Back to reviews" }));
    await expect(c.getByRole("list", { name: "Review threads" })).toBeVisible();
  }}
/>
