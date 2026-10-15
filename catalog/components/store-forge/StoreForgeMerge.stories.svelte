<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Merge footer",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
</script>

<Story
  name="Merge methods beneath collapsed checks"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const pr = await canvas.findByRole("region", {
      name: "Pull request for feature/api",
    });
    const footer = within(pr).getByRole("group", {
      name: "Pull request merge actions",
    });
    const checks = within(pr).getByRole("region", { name: "CI checks" });
    await expect(
      within(checks).getByRole("button", { name: "Expand checks" }),
    ).toHaveAttribute("aria-expanded", "false");
    await expect(footer.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      checks.getBoundingClientRect().bottom,
    );
    await expect(
      within(footer).getByRole("button", { name: "Squash and merge" }),
    ).toBeDisabled();
    await userEvent.click(
      within(footer).getByRole("button", { name: "Choose merge method" }),
    );
    const menu = page.getByRole("menu");
    for (const item of within(menu).getAllByRole("menuitemradio")) {
      await expect(item.querySelector("svg")).not.toBeNull();
      await expect(item.scrollWidth).toBeLessThanOrEqual(item.clientWidth + 1);
    }
    await userEvent.click(
      within(menu).getByRole("menuitemradio", { name: "Rebase and merge" }),
    );
    await expect(
      within(footer).getByRole("button", { name: "Rebase and merge" }),
    ).toBeDisabled();
    await userEvent.click(
      within(pr).getByRole("button", { name: "PR actions: Rebase" }),
    );
    await userEvent.click(
      page.getByRole("menuitem", { name: "Ready for review" }),
    );
    await expect(
      await within(footer).findByText("No conflicts with base branch"),
    ).toBeVisible();
    const merge = within(footer).getByRole("button", {
      name: "Rebase and merge",
    });
    await expect(merge).toBeEnabled();
    await userEvent.click(merge);
    const confirmation = await page.findByRole("dialog", {
      name: "Merge PR #42?",
    });
    await expect(
      within(confirmation).getByRole("button", { name: "Confirm merge" }),
    ).toBeEnabled();
    await userEvent.click(
      within(confirmation).getByRole("button", { name: /^Cancel$/ }),
    );
    await waitFor(() =>
      expect(page.queryByRole("dialog", { name: "Merge PR #42?" })).toBeNull(),
    );
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(
      within(pr).getByRole("button", { name: "Float PR details" }),
    );
    const floating = await page.findByRole("dialog", {
      name: "PR #42 · feature/api",
    });
    await expect(
      within(floating).getByRole("group", {
        name: "Pull request merge actions",
      }),
    ).toBe(footer);
    await userEvent.click(
      within(floating).getByRole("button", { name: "Redock floating pane" }),
    );
    await expect(
      within(pr).getByRole("group", { name: "Pull request merge actions" }),
    ).toBe(footer);
  }}
/>
