<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AppearanceOverlayFixture.svelte";
  import { buzzThemeNames } from "../../../web/src/lib/appearance/theme-catalog.js";
  const { Story } = defineMeta({
    title: "JJ Store/Appearance/Overlay tokens",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte",
      },
      layout: "fullscreen",
    },
  });
  async function verifyScopeStates(canvasElement: HTMLElement) {
    const p = within(canvasElement.ownerDocument.body);
    const dialog = await p.findByRole("dialog", { name: "Search workspace" });
    const c = within(dialog);
    const input = c.getByRole("combobox", { name: "Search workspace" });
    const scope = c.getByRole("option", { name: /Search in Projects/ });
    const rest = getComputedStyle(
      canvasElement.querySelector(".scope-rest-sample")!,
    ).backgroundColor;
    const active = getComputedStyle(
      canvasElement.querySelector(".scope-active-sample")!,
    ).backgroundColor;
    await userEvent.click(input);
    await userEvent.keyboard("{ArrowDown}{ArrowDown}");
    await waitFor(() => {
      expect(scope).toHaveAttribute("aria-selected", "false");
      expect(getComputedStyle(scope).backgroundColor).toBe(rest);
      expect(rest).not.toBe(active);
    });
    // Use a real browser pointer to exercise CSS :hover.
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer } = await import("vitest/browser");
      await pointer.hover(scope);
      await waitFor(() =>
        expect(getComputedStyle(scope).backgroundColor).toBe(active),
      );
      await pointer.hover(input);
    }
    await userEvent.keyboard("{ArrowUp}");
    await waitFor(() => {
      expect(scope).toHaveAttribute("aria-selected", "true");
      expect(getComputedStyle(scope).backgroundColor).toBe(active);
    });
    await userEvent.keyboard("{ArrowDown}");
    await waitFor(() =>
      expect(getComputedStyle(scope).backgroundColor).toBe(rest),
    );
  }
</script>

<Story
  name="Softer scope row with hover and keyboard selection in Buzz"
  args={{ initialTheme: "buzz" }}
  play={async ({ canvasElement }) => verifyScopeStates(canvasElement)}
/>
<Story
  name="Softer scope row with hover and keyboard selection in dark"
  args={{ initialTheme: "github-dark-default" }}
  play={async ({ canvasElement }) => verifyScopeStates(canvasElement)}
/>

<Story
  name="Search and menu share the app surface across all generated palettes"
  play={async ({ canvasElement }) => {
    const p = within(canvasElement.ownerDocument.body);
    const sample = canvasElement.querySelector(".appearance-overlay-fixture")!;
    const dialog = await p.findByRole("dialog", { name: "Search workspace" });
    const command = dialog.querySelector('[data-ui-component="command-view"]')!;
    const input = within(dialog).getByPlaceholderText(
      "Search threads, projects, tasks, repositories and code",
    );
    for (const theme of buzzThemeNames) {
      await waitFor(() => {
        expect(document.documentElement.dataset.storeTheme).toBe(theme);
        expect(getComputedStyle(dialog).backgroundColor, theme).toBe(
          getComputedStyle(sample).backgroundColor,
        );
        expect(getComputedStyle(command).backgroundColor, theme).toBe(
          getComputedStyle(sample).backgroundColor,
        );
      });
      await expect(input).toHaveValue("native");
      await waitFor(() =>
        expect(
          within(dialog).getByText("Retained search result"),
        ).toBeVisible(),
      );
      await userEvent.click(
        within(dialog).getByRole("button", { name: "Menu surface" }),
      );
      const menu = await p.findByRole("menu");
      await waitFor(() =>
        expect(getComputedStyle(menu).backgroundColor, theme).toBe(
          getComputedStyle(sample).backgroundColor,
        ),
      );
      await userEvent.keyboard("{Escape}");
      await userEvent.click(
        within(dialog).getByRole("button", { name: "Next palette" }),
      );
    }
  }}
/>
