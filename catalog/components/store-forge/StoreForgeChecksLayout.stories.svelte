<script lang="ts" module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Grouped checks",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending", "application"],
    args: { manyChecks: true },
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
    beforeEach: async () => {
      await views.stacks();
    },
  });
  async function inspect(canvasElement: HTMLElement) {
    const canvas = within(canvasElement);
    const pr = await canvas.findByRole("region", {
      name: "Pull request for feature/api",
    });
    await userEvent.click(
      within(pr).getByRole("button", { name: "Expand checks" }),
    );
    return await within(pr).findByRole("region", { name: "CI checks" });
  }
</script>

<Story
  name="Bounded status groups beside stacks"
  play={async ({ canvasElement }) => {
    const checks = await inspect(canvasElement);
    const c = within(checks);
    await expect(await c.findAllByText("Failing after 5s")).toHaveLength(2);
    const row = checks.querySelector<HTMLElement>(".forge-checks__row")!;
    const title = row.querySelector("strong")!,
      status = row.querySelector(".forge-checks__name > span")!;
    await expect(
      Math.abs(
        title.getBoundingClientRect().top - status.getBoundingClientRect().top,
      ),
    ).toBeLessThan(5);
    await expect(within(row).getAllByRole("button")).toHaveLength(1);

    await expect(
      await c.findByRole("button", { name: "2 failing checks" }),
    ).toBeVisible();
    await expect(
      c
        .getAllByRole("button", { name: /checks$/ })
        .filter((button) =>
          button.classList.contains("forge-checks__group-heading"),
        )
        .map((button) => button.textContent?.trim().replace(/\s+/g, " ")),
    ).toEqual([
      "2 failing checks",
      "2 in progress checks",
      "2 needing attention checks",
      "28 successful checks",
      "2 skipped or cancelled checks",
    ]);
    const scroll = checks.querySelector<HTMLElement>(".forge-checks__scroll")!;
    await waitFor(() =>
      expect(scroll.getBoundingClientRect().height).toBeLessThanOrEqual(
        Math.min(320, window.innerHeight * 0.45) + 1,
      ),
    );
    const viewport = scroll.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await expect(viewport.scrollHeight).toBeGreaterThan(viewport.clientHeight);
    await userEvent.click(c.getByRole("button", { name: "2 failing checks" }));
    await expect(
      c.getByRole("button", { name: "2 failing checks" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      c.getByRole("button", { name: "Check display options" }),
    );
    await userEvent.click(
      within(canvasElement.ownerDocument.body).getByRole("menuitemradio", {
        name: "No grouping",
      }),
    );
    await expect(
      c.queryByRole("button", { name: "2 failing checks" }),
    ).not.toBeInTheDocument();
    await expect(
      c.getByText("Workflow / job 36", { exact: true }),
    ).toBeInTheDocument();
    await userEvent.click(
      c.getByRole("button", { name: "Toggle check results" }),
    );
    await expect(
      c.getByRole("button", { name: "Toggle check results" }),
    ).toHaveAttribute("aria-expanded", "false");
  }}
/>
<Story
  name="Grouped checks in dark mode"
  globals={{ colorMode: "dark" }}
  args={{ colorMode: "dark" }}
  play={async ({ canvasElement }) => {
    await inspect(canvasElement);
    await expect(document.documentElement).toHaveClass("dark");
  }}
/>

<Story
  name="Grouped checks retain state through refresh and floating"
  play={async ({ canvasElement }) => {
    const checks = await inspect(canvasElement);
    const section = within(canvasElement).getByRole("region", {
      name: "Pull request for feature/api",
    });
    const c = within(checks);
    await userEvent.click(c.getByRole("button", { name: "2 failing checks" }));
    const viewport = checks.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    viewport.scrollTop = 160;
    viewport.dispatchEvent(new Event("scroll"));
    await userEvent.click(c.getByRole("button", { name: "Refresh checks" }));
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Refresh checks" })).toBeEnabled(),
    );
    await waitFor(() => expect(viewport.scrollTop).toBe(160));
    await expect(
      c.getByRole("button", { name: "2 failing checks" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      within(section).getByRole("button", { name: "Float PR details" }),
    );
    const floating = await within(canvasElement.ownerDocument.body).findByRole(
      "dialog",
      { name: "PR #42 · feature/api" },
    );
    await expect(
      within(floating).getByRole("region", { name: "CI checks" }),
    ).toBe(checks);
    await waitFor(() => expect(viewport.scrollTop, "after floating").toBe(160));
    if (import.meta.env.MODE === "test") {
      const { page: browser } = await import("vitest/browser");
      try {
        await browser.viewport(640, 640);
        await waitFor(() => {
          expect(viewport.getBoundingClientRect().height).toBeLessThanOrEqual(
            289,
          );
          expect(floating.getBoundingClientRect().right).toBeLessThanOrEqual(
            640,
          );
        });
      } finally {
        await browser.viewport(1280, 800);
      }
    }
    await userEvent.click(
      within(floating).getByRole("button", { name: "Redock floating pane" }),
    );
    await expect(
      within(section).getByRole("region", { name: "CI checks" }),
    ).toBe(checks);
    await waitFor(() =>
      expect(viewport.scrollTop, "after redocking").toBe(160),
    );
  }}
/>
